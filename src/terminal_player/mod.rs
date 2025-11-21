use crate::chip8::Chip8;
use crate::chip8::DISPLAY_PIXELS_X;
use crate::chip8::DISPLAY_PIXELS_Y;

pub struct TerminalPlayer {
    chip8: Chip8,
}

impl TerminalPlayer {
    pub fn new(chip8: Chip8) -> Self {
        Self { chip8 }
    }

    pub fn run(&mut self) {
        let program: [u8; 0] = [];
        self.chip8.initialize(&program);
        self.print_display();
    }

    fn print_display(&self) {
        let display_data = self.chip8.get_display();
        for i in 0..DISPLAY_PIXELS_Y {
            let mut line = String::new();
            for j in 0..DISPLAY_PIXELS_X {
                let pixel = i * DISPLAY_PIXELS_X + j;
                let byte = pixel >> 3;
                let bit = 7 - (pixel & 0x7);
                let char = match display_data[byte] & (1 << bit) {
                    0 => '.',
                    _ => 'X',
                };
                line.push(char);
            }
            println!("{}", line);
        }
    }
}
