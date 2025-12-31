use crate::assembler_error::{AssemblerError, LineLocation, Location};
use crate::statement::Statement;

use chip8_instructions::Instruction;
use itertools::Itertools;
use std::num::TryFromIntError;

pub fn parse_lines<'a>(text: &'a str) -> Result<Vec<Statement<'a>>, Vec<AssemblerError>> {
    let (oks, errs): (Vec<Option<Statement>>, Vec<AssemblerError>) = text
        .lines()
        .enumerate()
        .map(|(line_num, line)| parse_line(line, line_num as u32))
        .partition_result();
    if errs.len() > 0 {
        Err(errs)
    } else {
        Ok(oks.into_iter().flatten().collect())
    }
}

fn get_line_words(line: &str) -> Vec<&str> {
    line.split_whitespace()
        .take_while(|w| !w.contains("#"))
        .collect()
}

fn parse_line<'a>(line: &'a str, line_num: u32) -> Result<Option<Statement<'a>>, AssemblerError> {
    let words = get_line_words(line);

    if words.is_empty() {
        return Ok(None);
    }
    if words.len() == 1 && words[0].ends_with(":") {
        let label = words[0].chars().dropping_back(1).as_str();
        if label.is_empty() {
            return Err(AssemblerError::new_no_options(
                "Expected a label before the colon.".to_string(),
                Location::new(LineLocation::try_from_line_words(line, &words, 0), line_num),
            ));
        }
        return Ok(Some(Statement::Label {
            name: label,
            mem_addr: 0,
        }));
    }
    parse_instruction(line, line_num).map(|instruction| {
        Some(Statement::Instruction {
            instruction,
            mem_addr: 0,
        })
    })
}

fn parse_instruction(line: &str, line_num: u32) -> Result<Instruction, AssemblerError> {
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
        "JP" => get_jump(line, &words, line_num),
        _ => Err(AssemblerError::new_no_options(
            "Unknown instruction.".to_string(),
            Location::new_line_num(line_num),
        )),
    }
}

fn get_jump(line: &str, words: &Vec<&str>, line_num: u32) -> Result<Instruction, AssemblerError> {
    if words.len() != 2 {
        return Err(AssemblerError::new_no_options(
            "Expected one address for a JMP instruction".to_string(),
            Location::new_no_options(LineLocation::new(0, line.len() as u32), line_num),
        ));
    }
    Ok(Instruction::Jump {
        addr: try_parse_u16_literal(words[1])?,
    })
}

fn try_parse_number_literal(n_str: &str) -> Result<u64, AssemblerError> {
    let radix = match &n_str[0..2] {
        "0x" => 16,
        "0b" => 2,
        _ => 10,
    };
    u64::from_str_radix(n_str, radix).map_err(|error| {
        AssemblerError::new_message(format!("Failed to parse number: {:?}", error))
    })
}

fn try_parse_u16_literal(n_str: &str) -> Result<u16, AssemblerError> {
    try_parse_number_literal(n_str)?
        .try_into()
        .map_err(|error: TryFromIntError| {
            AssemblerError::new_message(format!("Failed to parse number: {:?}", error))
        })
}
