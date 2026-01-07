use crate::statement::*;

use chip8_instructions::Instruction;

#[test]
fn test_bytes_bytes() {
    let expected_bytes = vec![0xFF, 0xBB, 0x77, 0x00];
    let bytes_statement = Statement {
        mem_addr: 0,
        line_num: 0,
        line: String::new(),
        size: 0,
        statement_type: StatementType::Bytes {
            data: expected_bytes.clone(),
        },
    };
    assert_eq!(expected_bytes, bytes_statement.bytes());
}

#[test]
fn test_instruction_bytes() {
    let instruction_statement = Statement {
        mem_addr: 0,
        line_num: 0,
        line: String::new(),
        size: 0,
        statement_type: StatementType::Instruction {
            instruction: Instruction::Exit,
        },
    };
    assert_eq!(vec![0x00, 0xFD], instruction_statement.bytes());
}

#[test]
fn test_label_bytes() {
    let label_statement = Statement {
        mem_addr: 0,
        line_num: 0,
        line: String::new(),
        size: 0,
        statement_type: StatementType::Label { name: "label".to_string() },
    };
    assert!(label_statement.bytes().is_empty());
}
