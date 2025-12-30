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
