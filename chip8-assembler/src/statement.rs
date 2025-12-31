use chip8_instructions::Instruction;

#[derive(Clone, Copy)]
pub enum Statement<'a> {
    Label {
        name: &'a str,
        mem_addr: u16,
    },
    Instruction {
        instruction: Instruction,
        mem_addr: u16,
    },
}

impl<'a> Statement<'a> {
    pub fn set_mem_addr(&mut self, new_mem_addr: u16) {
        *self = match self {
            Statement::Label { name, .. } => Statement::Label {
                name,
                mem_addr: new_mem_addr,
            },
            Statement::Instruction { instruction, .. } => Statement::Instruction {
                instruction: *instruction,
                mem_addr: new_mem_addr,
            },
        };
    }
}
