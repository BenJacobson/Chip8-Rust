use chip8_instructions::*;

#[derive(Clone)]
pub struct Statement {
    pub mem_addr: u16,
    pub line_num: u16,
    pub size: u16,
    pub line: String,
    pub statement_type: StatementType,
}

#[derive(Clone)]
pub enum StatementType {
    Bytes { data: Vec<u8> },
    Instruction { instruction: Instruction },
    Label { name: String },
}

impl Statement {
    pub fn bytes(&self) -> Vec<u8> {
        match self.statement_type {
            StatementType::Bytes { ref data } => data.clone(),
            StatementType::Instruction { instruction } => {
                let (byte1, byte2) = encode_instruction(instruction);
                vec![byte1, byte2]
            }
            StatementType::Label { .. } => Vec::new(),
        }
    }
}
