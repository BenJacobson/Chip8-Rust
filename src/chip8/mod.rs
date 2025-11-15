#[derive(Debug)]
pub struct Chip8 {
    registers: [u8; 16],
    memory: [u8; 4096],
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            registers: [0; 16],
            memory: [0; 4096],
        }
    }
}
