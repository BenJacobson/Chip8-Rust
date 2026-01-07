use crate::*;

#[test]
fn test_null_instruction_decodes_to_unknown() {
    let instruction = decode_instruction(0x00, 0x00);
    let expected_instruction = Instruction::Unknown {
        byte1: 0x00,
        byte2: 0x00,
    };
    assert_eq!(expected_instruction, instruction);
}

// TODO: test decode for all instructions

#[test]
fn test_encode_unknown_instruction() {
    let instruction = Instruction::Unknown {
        byte1: 0x00,
        byte2: 0x00,
    };
    let (byte1, byte2) = encode_instruction(instruction);
    assert_eq!(0x00, byte1);
    assert_eq!(0x00, byte2);
}

#[test]
fn test_encode_clear_display() {
    let instruction = Instruction::ClearDisplay;
    let (byte1, byte2) = encode_instruction(instruction);
    assert_eq!(0x00, byte1);
    assert_eq!(0xE0, byte2);
}

#[test]
fn test_encode_return() {
    let instruction = Instruction::Return;
    let (byte1, byte2) = encode_instruction(instruction);
    assert_eq!(0x00, byte1);
    assert_eq!(0xEE, byte2);
}

#[test]
fn test_encode_exit() {
    let instruction = Instruction::Exit;
    let (byte1, byte2) = encode_instruction(instruction);
    assert_eq!(0x00, byte1);
    assert_eq!(0xFD, byte2);
}

#[test]
fn test_encode_jump() {
    let instruction = Instruction::Jump { addr: 0x123 };
    let (byte1, byte2) = encode_instruction(instruction);
    assert_eq!(0x11, byte1);
    assert_eq!(0x23, byte2);
}

#[test]
fn test_encode_call() {
    let instruction = Instruction::Call { addr: 0x456 };
    let (byte1, byte2) = encode_instruction(instruction);
    assert_eq!(0x24, byte1);
    assert_eq!(0x56, byte2);
}

#[test]
fn test_encode_skip_reg_equals_imm() {
    let instruction = Instruction::SkipRegEqualsImm { x: 0xA, byte: 0xBC };
    let (byte1, byte2) = encode_instruction(instruction);
    assert_eq!(0x3A, byte1);
    assert_eq!(0xBC, byte2);
}

#[test]
fn test_encode_skip_reg_not_equals_imm() {
    let instruction = Instruction::SkipRegNotEqualsImm { x: 0xD, byte: 0xEF };
    let (byte1, byte2) = encode_instruction(instruction);
    assert_eq!(0x4D, byte1);
    assert_eq!(0xEF, byte2);
}

#[test]
fn test_encode_skip_reg_equals_reg() {
    let instruction = Instruction::SkipRegEqualsReg { x: 0x1, y: 0x2 };
    let (byte1, byte2) = encode_instruction(instruction);
    assert_eq!(0x51, byte1);
    assert_eq!(0x20, byte2);
}

#[test]
fn test_encode_load_imm_to_reg() {
    let instruction = Instruction::LoadImmToReg { x: 0x3, byte: 0x45 };
    let (byte1, byte2) = encode_instruction(instruction);
    assert_eq!(0x63, byte1);
    assert_eq!(0x45, byte2);
}

#[test]
fn test_encode_add_imm_to_reg() {
    let instruction = Instruction::AddImmToReg { x: 0x6, byte: 0x78 };
    let (byte1, byte2) = encode_instruction(instruction);
    assert_eq!(0x76, byte1);
    assert_eq!(0x78, byte2);
}

#[test]
fn test_encode_load_reg_to_reg() {
    let instruction = Instruction::LoadRegToReg { x: 0x9, y: 0xA };
    let (byte1, byte2) = encode_instruction(instruction);
    assert_eq!(0x89, byte1);
    assert_eq!(0xA0, byte2);
}

#[test]
fn test_encode_or_reg() {
    let instruction = Instruction::OrReg { x: 0xB, y: 0xC };
    let (byte1, byte2) = encode_instruction(instruction);
    assert_eq!(0x8B, byte1);
    assert_eq!(0xC1, byte2);
}

#[test]
fn test_encode_and_reg() {
    let instruction = Instruction::AndReg { x: 0xD, y: 0xE };
    let (byte1, byte2) = encode_instruction(instruction);
    assert_eq!(0x8D, byte1);
    assert_eq!(0xE2, byte2);
}

#[test]
fn test_encode_xor_reg() {
    let instruction = Instruction::XorReg { x: 0xF, y: 0x0 };
    let (byte1, byte2) = encode_instruction(instruction);
    assert_eq!(0x8F, byte1);
    assert_eq!(0x03, byte2);
}

#[test]
fn test_encode_add_reg() {
    let instruction = Instruction::AddReg { x: 0x1, y: 0x2 };
    let (byte1, byte2) = encode_instruction(instruction);
    assert_eq!(0x81, byte1);
    assert_eq!(0x24, byte2);
}

#[test]
fn test_encode_sub_reg() {
    let instruction = Instruction::SubReg { x: 0x3, y: 0x4 };
    let (byte1, byte2) = encode_instruction(instruction);
    assert_eq!(0x83, byte1);
    assert_eq!(0x45, byte2);
}

#[test]
fn test_encode_shift_right() {
    let instruction = Instruction::ShiftRight { x: 0x5 };
    let (byte1, byte2) = encode_instruction(instruction);
    assert_eq!(0x85, byte1);
    assert_eq!(0x06, byte2);
}

#[test]
fn test_encode_sub_neg_reg() {
    let instruction = Instruction::SubNegReg { x: 0x6, y: 0x7 };
    let (byte1, byte2) = encode_instruction(instruction);
    assert_eq!(0x86, byte1);
    assert_eq!(0x77, byte2);
}

#[test]
fn test_encode_shift_left() {
    let instruction = Instruction::ShiftLeft { x: 0x8 };
    let (byte1, byte2) = encode_instruction(instruction);
    assert_eq!(0x88, byte1);
    assert_eq!(0x0E, byte2);
}

#[test]
fn test_encode_skip_reg_not_equals_reg() {
    let instruction = Instruction::SkipRegNotEqualsReg { x: 0x9, y: 0xA };
    let (byte1, byte2) = encode_instruction(instruction);
    assert_eq!(0x99, byte1);
    assert_eq!(0xA0, byte2);
}

#[test]
fn test_encode_load_imm_to_pointer() {
    let instruction = Instruction::LoadImmToPointer { addr: 0xBCD };
    let (byte1, byte2) = encode_instruction(instruction);
    assert_eq!(0xAB, byte1);
    assert_eq!(0xCD, byte2);
}

#[test]
fn test_encode_jump_offset() {
    let instruction = Instruction::JumpOffset { addr: 0xDEF };
    let (byte1, byte2) = encode_instruction(instruction);
    assert_eq!(0xBD, byte1);
    assert_eq!(0xEF, byte2);
}

#[test]
fn test_encode_random() {
    let instruction = Instruction::Random { x: 0x1, byte: 0x23 };
    let (byte1, byte2) = encode_instruction(instruction);
    assert_eq!(0xC1, byte1);
    assert_eq!(0x23, byte2);
}

#[test]
fn test_encode_draw() {
    let instruction = Instruction::Draw { x: 0x4, y: 0x5, nibble: 0x6 };
    let (byte1, byte2) = encode_instruction(instruction);
    assert_eq!(0xD4, byte1);
    assert_eq!(0x56, byte2);
}

#[test]
fn test_encode_skip_key_pressed() {
    let instruction = Instruction::SkipKeyPressed { x: 0x7 };
    let (byte1, byte2) = encode_instruction(instruction);
    assert_eq!(0xE7, byte1);
    assert_eq!(0x9E, byte2);
}

#[test]
fn test_encode_skip_not_key_pressed() {
    let instruction = Instruction::SkipNotKeyPressed { x: 0x8 };
    let (byte1, byte2) = encode_instruction(instruction);
    assert_eq!(0xE8, byte1);
    assert_eq!(0xA1, byte2);
}

#[test]
fn test_encode_load_delay_timer_to_reg() {
    let instruction = Instruction::LoadDelayTimerToReg { x: 0x9 };
    let (byte1, byte2) = encode_instruction(instruction);
    assert_eq!(0xF9, byte1);
    assert_eq!(0x07, byte2);
}

#[test]
fn test_encode_load_next_key_press() {
    let instruction = Instruction::LoadNextKeyPress { x: 0xA };
    let (byte1, byte2) = encode_instruction(instruction);
    assert_eq!(0xFA, byte1);
    assert_eq!(0x0A, byte2);
}

#[test]
fn test_encode_load_reg_to_delay_timer() {
    let instruction = Instruction::LoadRegToDelayTimer { x: 0xB };
    let (byte1, byte2) = encode_instruction(instruction);
    assert_eq!(0xFB, byte1);
    assert_eq!(0x15, byte2);
}

#[test]
fn test_encode_load_reg_to_sound_timer() {
    let instruction = Instruction::LoadRegToSoundTimer { x: 0xC };
    let (byte1, byte2) = encode_instruction(instruction);
    assert_eq!(0xFC, byte1);
    assert_eq!(0x18, byte2);
}

#[test]
fn test_encode_add_reg_to_pointer() {
    let instruction = Instruction::AddRegToPointer { x: 0xD };
    let (byte1, byte2) = encode_instruction(instruction);
    assert_eq!(0xFD, byte1);
    assert_eq!(0x1E, byte2);
}

#[test]
fn test_encode_load_digit_sprite_to_pointer() {
    let instruction = Instruction::LoadDigitSpriteToPointer { x: 0xE };
    let (byte1, byte2) = encode_instruction(instruction);
    assert_eq!(0xFE, byte1);
    assert_eq!(0x29, byte2);
}

#[test]
fn test_encode_load_decimal_digits_to_pointer() {
    let instruction = Instruction::LoadDecimalDigitsToPointer { x: 0xF };
    let (byte1, byte2) = encode_instruction(instruction);
    assert_eq!(0xFF, byte1);
    assert_eq!(0x33, byte2);
}

#[test]
fn test_encode_write_reg_to_pointer() {
    let instruction = Instruction::WriteRegToPointer { x: 0x0 };
    let (byte1, byte2) = encode_instruction(instruction);
    assert_eq!(0xF0, byte1);
    assert_eq!(0x55, byte2);
}

#[test]
fn test_encode_read_reg_from_pointer() {
    let instruction = Instruction::ReadRegFromPointer { x: 0x1 };
    let (byte1, byte2) = encode_instruction(instruction);
    assert_eq!(0xF1, byte1);
    assert_eq!(0x65, byte2);
}
