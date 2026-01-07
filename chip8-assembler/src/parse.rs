use crate::assembler_error::*;
use crate::statement::*;

use chip8_instructions::Instruction;
use itertools::Itertools;
use std::collections::HashMap;

pub fn get_partial_statement(line: &str, line_num: u16) -> Option<Statement> {
    let words = get_line_words(line);
    if words.is_empty() {
        return None;
    }

    let statement = match words[0] {
        word if words.len() == 1 && word.ends_with(":") => Statement {
            statement_type: StatementType::Label {
                name: String::new(),
            },
            size: 0,
            line_num,
            mem_addr: 0,
            line: line.to_string(),
        },
        word if try_parse_number_literal(word).is_ok() => Statement {
            statement_type: StatementType::Bytes { data: Vec::new() },
            size: words.len() as u16,
            line_num,
            mem_addr: 0,
            line: line.to_string(),
        },
        _ => Statement {
            statement_type: StatementType::Instruction {
                instruction: Instruction::Unknown { byte1: 0, byte2: 0 },
            },
            size: 2,
            line_num,
            mem_addr: 0,
            line: line.to_string(),
        },
    };
    Some(statement)
}

pub fn parse_label(line: &str, line_num: u16) -> Result<&str, AssemblerError> {
    let words = get_line_words(line);
    if words.len() == 1 && words[0].ends_with(":") {
        Ok(&words[0][..words[0].len()-1])
    } else {
        Err(AssemblerError::new_no_options(
            format!("Expected label, but got: {}", line),
            Location::new_line_num(line_num),
        ))
    }
}

pub fn parse_bytes(line: &str) -> Result<Vec<u8>, Vec<AssemblerError>> {
    let words = get_line_words(line);
    let (bytes, errors): (Vec<u8>, Vec<AssemblerError>) = words
        .iter()
        .map(|word| try_parse_byte_literal(word))
        .partition_result();
    if words.len() == bytes.len() {
        Ok(bytes)
    } else {
        Err(errors)
    }
}

pub fn parse_instruction(
    line: &str,
    line_num: u16,
    labels: &HashMap<String, u16>,
    mem_addr_max: u16,
) -> Result<Instruction, AssemblerError> {
    let words = get_line_words(line);
    let Some(first_word) = words.first() else {
        return Err(AssemblerError::new_no_options(
            "Unknown instruction.".to_string(),
            Location::new_line_num(line_num),
        ));
    };
    match first_word.to_uppercase().as_str() {
        "CLS" => Ok(Instruction::ClearDisplay),
        "RET" => Ok(Instruction::Return),
        "EXIT" => Ok(Instruction::Exit),
        "JP" => get_jp(line, &words, line_num, labels, mem_addr_max),
        "CALL" => get_call(line, &words, line_num, mem_addr_max),
        "SE" => get_se(line, &words, line_num),
        "SNE" => get_sne(line, &words, line_num),
        "LD" => get_ld(line, &words, line_num, mem_addr_max),
        "ADD" => get_add(line, &words, line_num),
        "OR" => get_or(line, &words, line_num),
        "AND" => get_and(line, &words, line_num),
        "XOR" => get_xor(line, &words, line_num),
        "SUB" => get_sub(line, &words, line_num),
        "SHR" => get_shr(line, &words, line_num),
        "SUBN" => get_subn(line, &words, line_num),
        "SHL" => get_shl(line, &words, line_num),
        "RND" => get_rnd(line, &words, line_num),
        "DRW" => get_drw(line, &words, line_num),
        "SKP" => get_skp(line, &words, line_num),
        "SKNP" => get_sknp(line, &words, line_num),
        _ => Err(AssemblerError::new_no_options(
            format!("Unknown instruction: {}", first_word),
            Location::new_line_num(line_num),
        )),
    }
}

/// Break lines into relevant words.
/// Comments and commas are removed.
fn get_line_words(mut line: &str) -> Vec<&str> {
    for comment_start in COMMENT_STARTERS.iter() {
        line = line.split(comment_start).next().unwrap_or("");
    }
    line.split_whitespace()
        .map(|word| {
            if word.ends_with(',') {
                &word[..word.len() - 1]
            } else {
                word
            }
        })
        .collect()
}

fn get_jp(
    line: &str,
    words: &Vec<&str>,
    line_num: u16,
    labels: &HashMap<String, u16>,
    mem_addr_max: u16,
) -> Result<Instruction, AssemblerError> {
    let get_addr = |word: &str| {
        if let Some(addr) = labels.get(word) {
            return Ok(*addr);
        }
        try_parse_addr_literal(word, mem_addr_max)
    };
    if words.len() == 2 {
        let addr = get_addr(words[1])?;
        return Ok(Instruction::Jump { addr });
    }
    if words.len() == 3 {
        if words[1].to_uppercase() == "V0" {
            let addr = get_addr(words[2])?;
            return Ok(Instruction::JumpOffset { addr });
        }
    }

    Err(AssemblerError::new_no_options(
        "Expected one address for a JP instruction, or V0 and an address.".to_string(),
        Location::new_no_options(LineLocation::new(0, line.len() as u32), line_num),
    ))
}

fn get_call(
    line: &str,
    words: &Vec<&str>,
    line_num: u16,
    mem_addr_max: u16,
) -> Result<Instruction, AssemblerError> {
    if words.len() != 2 {
        return Err(AssemblerError::new_no_options(
            "Expected one address for a CALL instruction".to_string(),
            Location::new_no_options(LineLocation::new(0, line.len() as u32), line_num),
        ));
    }
    Ok(Instruction::Call {
        addr: try_parse_addr_literal(words[1], mem_addr_max)?,
    })
}

fn get_se(line: &str, words: &Vec<&str>, line_num: u16) -> Result<Instruction, AssemblerError> {
    if words.len() != 3 {
        return Err(AssemblerError::new_no_options(
            "Expected two arguments for a SE instruction".to_string(),
            Location::new_no_options(LineLocation::new(0, line.len() as u32), line_num),
        ));
    }
    let x = try_parse_register(words[1])?;

    if let Ok(y) = try_parse_register(words[2]) {
        return Ok(Instruction::SkipRegEqualsReg { x, y });
    }

    let byte = try_parse_byte_literal(words[2])?;
    Ok(Instruction::SkipRegEqualsImm { x, byte })
}

fn get_sne(line: &str, words: &Vec<&str>, line_num: u16) -> Result<Instruction, AssemblerError> {
    if words.len() != 3 {
        return Err(AssemblerError::new_no_options(
            "Expected two arguments for a SNE instruction".to_string(),
            Location::new_no_options(LineLocation::new(0, line.len() as u32), line_num),
        ));
    }
    let x = try_parse_register(words[1])?;

    if let Ok(y) = try_parse_register(words[2]) {
        return Ok(Instruction::SkipRegNotEqualsReg { x, y });
    }

    let byte = try_parse_byte_literal(words[2])?;
    Ok(Instruction::SkipRegNotEqualsImm { x, byte })
}

fn get_ld(
    line: &str,
    words: &Vec<&str>,
    line_num: u16,
    mem_addr_max: u16,
) -> Result<Instruction, AssemblerError> {
    if words.len() != 3 {
        return Err(AssemblerError::new_no_options(
            "Expected two arguments for a LD instruction".to_string(),
            Location::new_no_options(LineLocation::new(0, line.len() as u32), line_num),
        ));
    }

    let arg1 = words[1];
    let arg2 = words[2];

    if let Ok(x) = try_parse_register(arg1) {
        if arg2.to_uppercase() == "DT" {
            return Ok(Instruction::LoadDelayTimerToReg { x });
        }
        if arg2.to_uppercase() == "K" {
            return Ok(Instruction::LoadNextKeyPress { x });
        }
        if arg2.to_uppercase() == "[I]" {
            return Ok(Instruction::ReadRegFromPointer { x });
        }

        if let Ok(y) = try_parse_register(arg2) {
            return Ok(Instruction::LoadRegToReg { x, y });
        }

        if let Ok(byte) = try_parse_byte_literal(arg2) {
            return Ok(Instruction::LoadImmToReg { x, byte });
        }

        return Err(AssemblerError::new_no_options(
            format!("Invalid second argument for LD: {}", arg2),
            Location::new_line_num(line_num),
        ));
    }

    if arg1.to_uppercase() == "I" {
        let addr = try_parse_addr_literal(arg2, mem_addr_max)?;
        return Ok(Instruction::LoadImmToPointer { addr });
    }

    if arg1.to_uppercase() == "DT" {
        let x = try_parse_register(arg2)?;
        return Ok(Instruction::LoadRegToDelayTimer { x });
    }

    if arg1.to_uppercase() == "ST" {
        let x = try_parse_register(arg2)?;
        return Ok(Instruction::LoadRegToSoundTimer { x });
    }

    if arg1.to_uppercase() == "F" {
        let x = try_parse_register(arg2)?;
        return Ok(Instruction::LoadDigitSpriteToPointer { x });
    }

    if arg1.to_uppercase() == "B" {
        let x = try_parse_register(arg2)?;
        return Ok(Instruction::LoadDecimalDigitsToPointer { x });
    }

    if arg1.to_uppercase() == "[I]" {
        let x = try_parse_register(arg2)?;
        return Ok(Instruction::WriteRegToPointer { x });
    }

    Err(AssemblerError::new_no_options(
        format!("Invalid first argument for LD: {}", arg1),
        Location::new_line_num(line_num),
    ))
}

fn get_add(line: &str, words: &Vec<&str>, line_num: u16) -> Result<Instruction, AssemblerError> {
    if words.len() != 3 {
        return Err(AssemblerError::new_no_options(
            "Expected two arguments for an ADD instruction".to_string(),
            Location::new_no_options(LineLocation::new(0, line.len() as u32), line_num),
        ));
    }

    if words[1].to_uppercase() == "I" {
        let x = try_parse_register(words[2])?;
        return Ok(Instruction::AddRegToPointer { x });
    }

    let x = try_parse_register(words[1])?;

    if let Ok(y) = try_parse_register(words[2]) {
        return Ok(Instruction::AddReg { x, y });
    }

    let byte = try_parse_byte_literal(words[2])?;
    Ok(Instruction::AddImmToReg { x, byte })
}

fn get_or(line: &str, words: &Vec<&str>, line_num: u16) -> Result<Instruction, AssemblerError> {
    if words.len() != 3 {
        return Err(AssemblerError::new_no_options(
            "Expected two register arguments for an OR instruction".to_string(),
            Location::new_no_options(LineLocation::new(0, line.len() as u32), line_num),
        ));
    }
    let x = try_parse_register(words[1])?;
    let y = try_parse_register(words[2])?;
    Ok(Instruction::OrReg { x, y })
}

fn get_and(line: &str, words: &Vec<&str>, line_num: u16) -> Result<Instruction, AssemblerError> {
    if words.len() != 3 {
        return Err(AssemblerError::new_no_options(
            "Expected two register arguments for an AND instruction".to_string(),
            Location::new_no_options(LineLocation::new(0, line.len() as u32), line_num),
        ));
    }
    let x = try_parse_register(words[1])?;
    let y = try_parse_register(words[2])?;
    Ok(Instruction::AndReg { x, y })
}

fn get_xor(line: &str, words: &Vec<&str>, line_num: u16) -> Result<Instruction, AssemblerError> {
    if words.len() != 3 {
        return Err(AssemblerError::new_no_options(
            "Expected two register arguments for a XOR instruction".to_string(),
            Location::new_no_options(LineLocation::new(0, line.len() as u32), line_num),
        ));
    }
    let x = try_parse_register(words[1])?;
    let y = try_parse_register(words[2])?;
    Ok(Instruction::XorReg { x, y })
}

fn get_sub(line: &str, words: &Vec<&str>, line_num: u16) -> Result<Instruction, AssemblerError> {
    if words.len() != 3 {
        return Err(AssemblerError::new_no_options(
            "Expected two register arguments for a SUB instruction".to_string(),
            Location::new_no_options(LineLocation::new(0, line.len() as u32), line_num),
        ));
    }
    let x = try_parse_register(words[1])?;
    let y = try_parse_register(words[2])?;
    Ok(Instruction::SubReg { x, y })
}

fn get_shr(line: &str, words: &Vec<&str>, line_num: u16) -> Result<Instruction, AssemblerError> {
    if words.len() != 2 {
        return Err(AssemblerError::new_no_options(
            "Expected one register argument for a SHR instruction".to_string(),
            Location::new_no_options(LineLocation::new(0, line.len() as u32), line_num),
        ));
    }
    let x = try_parse_register(words[1])?;
    Ok(Instruction::ShiftRight { x })
}

fn get_subn(line: &str, words: &Vec<&str>, line_num: u16) -> Result<Instruction, AssemblerError> {
    if words.len() != 3 {
        return Err(AssemblerError::new_no_options(
            "Expected two register arguments for a SUBN instruction".to_string(),
            Location::new_no_options(LineLocation::new(0, line.len() as u32), line_num),
        ));
    }
    let x = try_parse_register(words[1])?;
    let y = try_parse_register(words[2])?;
    Ok(Instruction::SubNegReg { x, y })
}

fn get_shl(line: &str, words: &Vec<&str>, line_num: u16) -> Result<Instruction, AssemblerError> {
    if words.len() != 2 {
        return Err(AssemblerError::new_no_options(
            "Expected one register argument for a SHL instruction".to_string(),
            Location::new_no_options(LineLocation::new(0, line.len() as u32), line_num),
        ));
    }
    let x = try_parse_register(words[1])?;
    Ok(Instruction::ShiftLeft { x })
}

fn get_rnd(line: &str, words: &Vec<&str>, line_num: u16) -> Result<Instruction, AssemblerError> {
    if words.len() != 3 {
        return Err(AssemblerError::new_no_options(
            "Expected a register and a byte for a RND instruction".to_string(),
            Location::new_no_options(LineLocation::new(0, line.len() as u32), line_num),
        ));
    }
    let x = try_parse_register(words[1])?;
    let byte = try_parse_byte_literal(words[2])?;
    Ok(Instruction::Random { x, byte })
}

fn get_drw(line: &str, words: &Vec<&str>, line_num: u16) -> Result<Instruction, AssemblerError> {
    if words.len() != 4 {
        return Err(AssemblerError::new_no_options(
            "Expected two registers and a nibble for a DRW instruction".to_string(),
            Location::new_no_options(LineLocation::new(0, line.len() as u32), line_num),
        ));
    }
    let x = try_parse_register(words[1])?;
    let y = try_parse_register(words[2])?;
    let nibble = try_parse_nibble_literal(words[3])?;
    Ok(Instruction::Draw { x, y, nibble })
}

fn get_skp(line: &str, words: &Vec<&str>, line_num: u16) -> Result<Instruction, AssemblerError> {
    if words.len() != 2 {
        return Err(AssemblerError::new_no_options(
            "Expected one register argument for a SKP instruction".to_string(),
            Location::new_no_options(LineLocation::new(0, line.len() as u32), line_num),
        ));
    }
    let x = try_parse_register(words[1])?;
    Ok(Instruction::SkipKeyPressed { x })
}

fn get_sknp(line: &str, words: &Vec<&str>, line_num: u16) -> Result<Instruction, AssemblerError> {
    if words.len() != 2 {
        return Err(AssemblerError::new_no_options(
            "Expected one register argument for a SKNP instruction".to_string(),
            Location::new_no_options(LineLocation::new(0, line.len() as u32), line_num),
        ));
    }
    let x = try_parse_register(words[1])?;
    Ok(Instruction::SkipNotKeyPressed { x })
}

fn try_parse_number_literal(n_str: &str) -> Result<u64, AssemblerError> {
    let (radix, s) = if n_str.to_lowercase().starts_with("0x") {
        (16, &n_str[2..])
    } else if n_str.to_lowercase().starts_with("0b") {
        (2, &n_str[2..])
    } else {
        (10, n_str)
    };

    if s.is_empty() {
        return Err(AssemblerError::new_message(format!(
            "Failed to parse empty number string."
        )));
    }

    u64::from_str_radix(s, radix).map_err(|error| {
        AssemblerError::new_message(format!("Failed to parse number: `{}`, {:?}", n_str, error))
    })
}

fn try_parse_addr_literal(n_str: &str, mem_addr_max: u16) -> Result<u16, AssemblerError> {
    let n = try_parse_number_literal(n_str)?;
    if n < mem_addr_max.into() {
        Ok(n as u16)
    } else {
        Err(AssemblerError::new_message(format!(
            "Address literal ({}) is larger than the available address space (0x{:x}).",
            n_str, mem_addr_max
        )))
    }
}

fn try_parse_register(reg_str: &str) -> Result<u8, AssemblerError> {
    if !reg_str.to_uppercase().starts_with('V') {
        return Err(AssemblerError::new_message(format!(
            "Expected a register of format (V?), but got ({}).",
            reg_str
        )));
    }
    let n_str = &reg_str[1..];
    if n_str.is_empty() {
        return Err(AssemblerError::new_message(format!(
            "Expecting a digit after 'V'"
        )));
    }
    let n = u8::from_str_radix(n_str, 16).map_err(|error| {
        AssemblerError::new_message(format!("Failed to parse register: {:?}, {}", error, n_str))
    })?;

    if n <= NUM_REGISTERS {
        Ok(n as u8)
    } else {
        Err(AssemblerError::new_message(format!(
            "Register literal ({}) is larger than the number of registers (0x{:x}).",
            reg_str, NUM_REGISTERS
        )))
    }
}

fn try_parse_byte_literal(n_str: &str) -> Result<u8, AssemblerError> {
    let n = try_parse_number_literal(n_str)?;
    if n <= u8::MAX.into() {
        Ok(n as u8)
    } else {
        Err(AssemblerError::new_message(format!(
            "Byte literal ({}) is larger than the max byte (0x{:x}).",
            n_str,
            u8::MAX
        )))
    }
}

fn try_parse_nibble_literal(n_str: &str) -> Result<u8, AssemblerError> {
    let n = try_parse_number_literal(n_str)?;
    if n <= 0xF {
        Ok(n as u8)
    } else {
        Err(AssemblerError::new_message(format!(
            "Nibble literal ({}) is larger than the max nibble (0xF).",
            n_str
        )))
    }
}

const COMMENT_STARTERS: [&str; 4] = [";", "//", "#", "@"];
const NUM_REGISTERS: u8 = 0xF;
