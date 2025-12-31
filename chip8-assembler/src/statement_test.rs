use crate::statement::*;

use chip8_instructions::Instruction;

#[test]
fn test_set_label_mem_addr() {
    let mut label = Statement::Label {
        name: "test",
        mem_addr: 0,
    };
    Statement::set_mem_addr(&mut label, 1);
    if let Statement::Label { mem_addr, .. } = label {
        assert_eq!(1, mem_addr)
    } else {
        panic!()
    }
}

#[test]
fn test_set_instruction_mem_addr() {
    let mut instruction = Statement::Instruction { instruction: Instruction::Exit, mem_addr: 0 } ;
    Statement::set_mem_addr(&mut instruction, 1);
    if let Statement::Instruction { mem_addr, .. } = instruction {
        assert_eq!(1, mem_addr)
    } else {
        panic!()
    }
}
