use itertools::Itertools;
use std::time::Duration;
use std::time::Instant;

use crate::chip8::Chip8;

pub struct TerminalPlayer {
    chip8: Chip8,
    prev_output: String,
}

const TICK_MICROS: u32 = 16666;

impl TerminalPlayer {
    pub fn new(chip8: Chip8) -> Self {
        Self {
            chip8,
            prev_output: String::new(),
        }
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

    fn print_display(&mut self) {
        let display = self.chip8.get_display();
        let output = (0..display.height)
            .map(|i| {
                (0..display.width)
                    .map(|j| display.get_pixel(i, j))
                    .map(|set| if set { '█' } else { '⠀' })
                    .collect::<String>()
            })
            .chain(["\n".to_string()])
            .join("\n");
        if output == self.prev_output {
            return;
        }
        println!("{}", output);
        self.prev_output = output;
    }
}
