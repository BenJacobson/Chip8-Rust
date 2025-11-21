use std::time::Duration;
use std::time::Instant;

use crate::chip8::Chip8;
use crate::chip8::DISPLAY_PIXELS_X;
use crate::chip8::DISPLAY_PIXELS_Y;

pub struct TerminalPlayer {
    chip8: Chip8,
}

const TICK_MICROS: u32 = 16666;

impl TerminalPlayer {
    pub fn new(chip8: Chip8) -> Self {
        Self { chip8 }
    }

    pub fn run(&mut self, program: &[u8]) {
        self.chip8.initialize(program);

        let mut instant = Instant::now();
        while !self.chip8.run_next_instruction() {
            let elapsed_micros = instant.elapsed().subsec_micros();
            if elapsed_micros >= TICK_MICROS {
                self.chip8.tick_timers();
                instant += Duration::from_micros(TICK_MICROS.into());
                self.print_display();
            }
        }
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
