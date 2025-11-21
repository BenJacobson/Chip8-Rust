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
        let program: [u8; 160] = [
            // 0
            0x60, 0x00, // Load register v0=0
            0xF0, 0x29, // LoadDigitSpriteToPointer v0=0
            0x61, 0x00, // Load register v1=0
            0x62, 0x00, // Load register v2=0
            0xD1, 0x25, // Draw a 5 byte sprite at v1=0, v2=0
            // 1
            0x60, 0x01, // Load register v0=1
            0xF0, 0x29, // LoadDigitSpriteToPointer v0=1
            0x61, 0x05, // Load register v1=5
            0x62, 0x00, // Load register v2=0
            0xD1, 0x25, // Draw a 5 byte sprite at v1=5, v2=0
            // 2
            0x60, 0x02, // Load register v0=2
            0xF0, 0x29, // LoadDigitSpriteToPointer v0=2
            0x61, 0x0A, // Load register v1=10
            0x62, 0x00, // Load register v2=0
            0xD1, 0x25, // Draw a 5 byte sprite at v1=10, v2=0
            // 3
            0x60, 0x03, // Load register v0=3
            0xF0, 0x29, // LoadDigitSpriteToPointer v0=3
            0x61, 0x0F, // Load register v1=15
            0x62, 0x00, // Load register v2=0
            0xD1, 0x25, // Draw a 5 byte sprite at v1=15, v2=0
            // 4
            0x60, 0x04, // Load register v0=4
            0xF0, 0x29, // LoadDigitSpriteToPointer v0=0
            0x61, 0x00, // Load register v1=0
            0x62, 0x06, // Load register v2=6
            0xD1, 0x25, // Draw a 5 byte sprite at v1=0, v2=6
            // 5
            0x60, 0x05, // Load register v0=5
            0xF0, 0x29, // LoadDigitSpriteToPointer v0=1
            0x61, 0x05, // Load register v1=5
            0x62, 0x06, // Load register v2=6
            0xD1, 0x25, // Draw a 5 byte sprite at v1=5, v2=6
            // 6
            0x60, 0x06, // Load register v0=6
            0xF0, 0x29, // LoadDigitSpriteToPointer v0=2
            0x61, 0x0A, // Load register v1=10
            0x62, 0x06, // Load register v2=6
            0xD1, 0x25, // Draw a 5 byte sprite at v1=10, v2=6
            // 7
            0x60, 0x07, // Load register v0=7
            0xF0, 0x29, // LoadDigitSpriteToPointer v0=3
            0x61, 0x0F, // Load register v1=15
            0x62, 0x06, // Load register v2=6
            0xD1, 0x25, // Draw a 5 byte sprite at v1=15, v2=6
            // 8
            0x60, 0x08, // Load register v0=8
            0xF0, 0x29, // LoadDigitSpriteToPointer v0=0
            0x61, 0x00, // Load register v1=0
            0x62, 0x0C, // Load register v2=C
            0xD1, 0x25, // Draw a 5 byte sprite at v1=0, v2=C
            // 9
            0x60, 0x09, // Load register v0=9
            0xF0, 0x29, // LoadDigitSpriteToPointer v0=1
            0x61, 0x05, // Load register v1=5
            0x62, 0x0C, // Load register v2=C
            0xD1, 0x25, // Draw a 5 byte sprite at v1=5, v2=C
            // A
            0x60, 0x0A, // Load register v0=A
            0xF0, 0x29, // LoadDigitSpriteToPointer v0=2
            0x61, 0x0A, // Load register v1=10
            0x62, 0x0C, // Load register v2=C
            0xD1, 0x25, // Draw a 5 byte sprite at v1=10, v2=C
            // B
            0x60, 0x0B, // Load register v0=B
            0xF0, 0x29, // LoadDigitSpriteToPointer v0=3
            0x61, 0x0F, // Load register v1=15
            0x62, 0x0C, // Load register v2=C
            0xD1, 0x25, // Draw a 5 byte sprite at v1=15, v2=C
            // C
            0x60, 0x0C, // Load register v0=C
            0xF0, 0x29, // LoadDigitSpriteToPointer v0=0
            0x61, 0x00, // Load register v1=0
            0x62, 0x12, // Load register v2=0x12
            0xD1, 0x25, // Draw a 5 byte sprite at v1=0, v2=0x12
            // D
            0x60, 0x0D, // Load register v0=D
            0xF0, 0x29, // LoadDigitSpriteToPointer v0=1
            0x61, 0x05, // Load register v1=5
            0x62, 0x12, // Load register v2=0x12
            0xD1, 0x25, // Draw a 5 byte sprite at v1=5, v2=0x12
            // E
            0x60, 0x0E, // Load register v0=E
            0xF0, 0x29, // LoadDigitSpriteToPointer v0=2
            0x61, 0x0A, // Load register v1=10
            0x62, 0x12, // Load register v2=0x12
            0xD1, 0x25, // Draw a 5 byte sprite at v1=10, v2=0x12
            // F
            0x60, 0x0F, // Load register v0=F
            0xF0, 0x29, // LoadDigitSpriteToPointer v0=3
            0x61, 0x0F, // Load register v1=15
            0x62, 0x12, // Load register v2=0x12
            0xD1, 0x25, // Draw a 5 byte sprite at v1=15, v2=0x12
        ];
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
                let char = if display_data[byte] & (1 << bit) == 0 {
                    '.'
                } else {
                    'X'
                };
                line.push(char);
            }
            println!("{}", line);
        }
    }
}
