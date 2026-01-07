use crate::parse::*;

use chip8_instructions::Instruction;

#[test]
fn test_parse_cls() {
    let line = "CLS";
    let instruction = parse_instruction(line, 0, 0xFFFF).unwrap();
    assert_eq!(instruction, Instruction::ClearDisplay);
}

#[test]
fn test_parse_ret() {
    let line = "RET";
    let instruction = parse_instruction(line, 0, 0xFFFF).unwrap();
    assert_eq!(instruction, Instruction::Return);
}

#[test]
fn test_parse_jp() {
    let line = "JP 1234";
    let instruction = parse_instruction(line, 0, 0xFFFF).unwrap();
    assert_eq!(instruction, Instruction::Jump { addr: 1234 });
}

#[test]
fn test_parse_call() {
    let line = "CALL 0x400";
    let instruction = parse_instruction(line, 0, 0xFFFF).unwrap();
    assert_eq!(instruction, Instruction::Call { addr: 0x400 });
}

#[test]
fn test_parse_se_imm() {
    let line = "SE V7, 2";
    let instruction = parse_instruction(line, 0, 0xFFFF).unwrap();
    assert_eq!(instruction, Instruction::SkipRegEqualsImm { x: 7, byte: 2 });
}

#[test]
fn test_parse_sne_imm() {
    let line = "SNE VA, 200";
    let instruction = parse_instruction(line, 0, 0xFFFF).unwrap();
    assert_eq!(
        instruction,
        Instruction::SkipRegNotEqualsImm { x: 0xA, byte: 200 }
    );
}

#[test]
fn test_parse_se_reg() {
    let line = "SE V1, V0";
    let instruction = parse_instruction(line, 0, 0xFFFF).unwrap();
    assert_eq!(instruction, Instruction::SkipRegEqualsReg { x: 1, y: 0 });
}

#[test]
fn test_parse_ld_reg_byte() {
    let line = "LD V1, 0x23";
    let instruction = parse_instruction(line, 0, 0xFFFF).unwrap();
    assert_eq!(instruction, Instruction::LoadImmToReg { x: 1, byte: 0x23 });
}

#[test]
fn test_parse_add_imm() {
    let line = "ADD V2, 5";
    let instruction = parse_instruction(line, 0, 0xFFFF).unwrap();
    assert_eq!(instruction, Instruction::AddImmToReg { x: 2, byte: 5 });
}

#[test]
fn test_parse_ld_reg_reg() {
    let line = "LD VA, VB";
    let instruction = parse_instruction(line, 0, 0xFFFF).unwrap();
    assert_eq!(instruction, Instruction::LoadRegToReg { x: 0xA, y: 0xB });
}

#[test]
fn test_parse_or() {
    let line = "OR V9, VE";
    let instruction = parse_instruction(line, 0, 0xFFFF).unwrap();
    assert_eq!(instruction, Instruction::OrReg { x: 0x9, y: 0xE });
}

#[test]
fn test_parse_and() {
    let line = "AND VF, V5";
    let instruction = parse_instruction(line, 0, 0xFFFF).unwrap();
    assert_eq!(instruction, Instruction::AndReg { x: 0xF, y: 0x5 });
}

#[test]
fn test_parse_xor() {
    let line = "XOR V8, V3";
    let instruction = parse_instruction(line, 0, 0xFFFF).unwrap();
    assert_eq!(instruction, Instruction::XorReg { x: 8, y: 3 });
}

#[test]
fn test_parse_add_reg() {
    let line = "ADD V2, V5";
    let instruction = parse_instruction(line, 0, 0xFFFF).unwrap();
    assert_eq!(instruction, Instruction::AddReg { x: 2, y: 5 });
}

#[test]
fn test_parse_sub() {
    let line = "SUB V4, VB";
    let instruction = parse_instruction(line, 0, 0xFFFF).unwrap();
    assert_eq!(instruction, Instruction::SubReg { x: 0x4, y: 0xB });
}

#[test]
fn test_parse_shift_right() {
    let line = "SHR V6";
    let instruction = parse_instruction(line, 0, 0xFFFF).unwrap();
    assert_eq!(instruction, Instruction::ShiftRight { x: 6 });
}

#[test]
fn test_parse_sub_neg() {
    let line = "SUBN V4, VB";
    let instruction = parse_instruction(line, 0, 0xFFFF).unwrap();
    assert_eq!(instruction, Instruction::SubNegReg { x: 0x4, y: 0xB });
}

#[test]
fn test_parse_shift_left() {
    let line = "SHL VC";
    let instruction = parse_instruction(line, 0, 0xFFFF).unwrap();
    assert_eq!(instruction, Instruction::ShiftLeft { x: 0xC });
}

#[test]
fn test_parse_sne_reg() {
    let line = "SNE VA, VB";
    let instruction = parse_instruction(line, 0, 0xFFFF).unwrap();
    assert_eq!(
        instruction,
        Instruction::SkipRegNotEqualsReg { x: 0xA, y: 0xB }
    );
}

#[test]
fn test_load_pointer() {
    let line = "LD I, 0xFED";
    let instruction = parse_instruction(line, 0, 0xFFFF).unwrap();
    assert_eq!(instruction, Instruction::LoadImmToPointer { addr: 0xFED });
}

#[test]
fn test_parse_jp_offset() {
    let line = "JP v0 0b101";
    let instruction = parse_instruction(line, 0, 0xFFFF).unwrap();
    assert_eq!(instruction, Instruction::JumpOffset { addr: 0b101 });
}

#[test]
fn test_parse_rnd() {
    let line = "RND v3 0b00001111";
    let instruction = parse_instruction(line, 0, 0xFFFF).unwrap();
    assert_eq!(
        instruction,
        Instruction::Random {
            x: 3,
            byte: 0b00001111
        }
    );
}

#[test]
fn test_parse_drw() {
    let line = "DRW V5, V6, 15";
    let instruction = parse_instruction(line, 0, 0xFFFF).unwrap();
    assert_eq!(
        instruction,
        Instruction::Draw {
            x: 5,
            y: 6,
            nibble: 15
        }
    );
}

#[test]
fn test_parse_skp() {
    let line = "SKP VD";
    let instruction = parse_instruction(line, 0, 0xFFFF).unwrap();
    assert_eq!(instruction, Instruction::SkipKeyPressed { x: 0xD });
}

#[test]
fn test_parse_sknp() {
    let line = "SKNP VD";
    let instruction = parse_instruction(line, 0, 0xFFFF).unwrap();
    assert_eq!(instruction, Instruction::SkipNotKeyPressed { x: 0xD });
}

#[test]
fn test_load_from_delay_timer() {
    let line = "LD v1 dt";
    let instruction = parse_instruction(line, 0, 0xFFFF).unwrap();
    assert_eq!(instruction, Instruction::LoadDelayTimerToReg { x: 1 });
}

#[test]
fn test_load_next_key_press() {
    let line = "LD v1 k";
    let instruction = parse_instruction(line, 0, 0xFFFF).unwrap();
    assert_eq!(instruction, Instruction::LoadNextKeyPress { x: 1 });
}

#[test]
fn test_load_to_delay_timer() {
    let line = "LD dt v4";
    let instruction = parse_instruction(line, 0, 0xFFFF).unwrap();
    assert_eq!(instruction, Instruction::LoadRegToDelayTimer { x: 4 });
}

#[test]
fn test_load_to_sound_timer() {
    let line = "LD ST v4";
    let instruction = parse_instruction(line, 0, 0xFFFF).unwrap();
    assert_eq!(instruction, Instruction::LoadRegToSoundTimer { x: 4 });
}

#[test]
fn test_add_reg_to_pointer() {
    let line = "ADD i, vA";
    let instruction = parse_instruction(line, 0, 0xFFFF).unwrap();
    assert_eq!(instruction, Instruction::AddRegToPointer { x: 0xa });
}

#[test]
fn test_load_sprite() {
    let line = "LD F, vE";
    let instruction = parse_instruction(line, 0, 0xFFFF).unwrap();
    assert_eq!(
        instruction,
        Instruction::LoadDigitSpriteToPointer { x: 0xe }
    );
}

#[test]
fn test_load_decimal_digits() {
    let line = "LD B, V9";
    let instruction = parse_instruction(line, 0, 0xFFFF).unwrap();
    assert_eq!(
        instruction,
        Instruction::LoadDecimalDigitsToPointer { x: 9 }
    );
}

#[test]
fn test_push_registers() {
    let line = "LD [I], vB";
    let instruction = parse_instruction(line, 0, 0xFFFF).unwrap();
    assert_eq!(instruction, Instruction::WriteRegToPointer { x: 0xb });
}

#[test]
fn test_pop_registers() {
    let line = "LD vB, [I]";
    let instruction = parse_instruction(line, 0, 0).unwrap();
    assert_eq!(instruction, Instruction::ReadRegFromPointer { x: 0xb });
}

#[test]
fn test_parse_bytes() {
    let line = "0xAB 12 0b00101010";
    let bytes_result = parse_bytes(line);
    let bytes = bytes_result.unwrap();
    assert_eq!(vec![0xABu8, 12u8, 0b00101010u8], bytes);
}

#[test]
fn test_error_parsing_number() {
    let Err(error) = parse_instruction("jp abc", 0, 0x100) else {
        panic!("Parsing statements failed to fail.");
    };
    assert!(error.message.find("Failed to parse number").is_some());
}
