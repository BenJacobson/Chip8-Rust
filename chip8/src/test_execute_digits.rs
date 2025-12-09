use super::processor::*;
use crate::logger::Logger;

static RENDER_DIGITS_PROGRAM: [u8; 162] = [
    // 0
    0x60, 0x00, // Load register v0=0
    0x61, 0x00, // Load register v1=0
    0x62, 0x00, // Load register v2=0
    0xF0, 0x29, // LoadDigitSpriteToPointer v0=0
    0xD1, 0x25, // Draw a 5 byte sprite at v1=0, v2=0
    // 1
    0x60, 0x01, // Load register v0=1
    0x61, 0x05, // Load register v1=5
    0x62, 0x00, // Load register v2=0
    0xF0, 0x29, // LoadDigitSpriteToPointer v0=1
    0xD1, 0x25, // Draw a 5 byte sprite at v1=5, v2=0
    // 2
    0x60, 0x02, // Load register v0=2
    0x61, 0x0A, // Load register v1=10
    0x62, 0x00, // Load register v2=0
    0xF0, 0x29, // LoadDigitSpriteToPointer v0=2
    0xD1, 0x25, // Draw a 5 byte sprite at v1=10, v2=0
    // 3
    0x60, 0x03, // Load register v0=3
    0x61, 0x0F, // Load register v1=15
    0x62, 0x00, // Load register v2=0
    0xF0, 0x29, // LoadDigitSpriteToPointer v0=3
    0xD1, 0x25, // Draw a 5 byte sprite at v1=15, v2=0
    // 4
    0x60, 0x04, // Load register v0=4
    0x61, 0x00, // Load register v1=0
    0x62, 0x06, // Load register v2=6
    0xF0, 0x29, // LoadDigitSpriteToPointer v0=0
    0xD1, 0x25, // Draw a 5 byte sprite at v1=0, v2=6
    // 5
    0x60, 0x05, // Load register v0=5
    0x61, 0x05, // Load register v1=5
    0x62, 0x06, // Load register v2=6
    0xF0, 0x29, // LoadDigitSpriteToPointer v0=1
    0xD1, 0x25, // Draw a 5 byte sprite at v1=5, v2=6
    // 6
    0x60, 0x06, // Load register v0=6
    0x61, 0x0A, // Load register v1=10
    0x62, 0x06, // Load register v2=6
    0xF0, 0x29, // LoadDigitSpriteToPointer v0=2
    0xD1, 0x25, // Draw a 5 byte sprite at v1=10, v2=6
    // 7
    0x60, 0x07, // Load register v0=7
    0x61, 0x0F, // Load register v1=15
    0x62, 0x06, // Load register v2=6
    0xF0, 0x29, // LoadDigitSpriteToPointer v0=3
    0xD1, 0x25, // Draw a 5 byte sprite at v1=15, v2=6
    // 8
    0x60, 0x08, // Load register v0=8
    0x61, 0x00, // Load register v1=0
    0x62, 0x0C, // Load register v2=C
    0xF0, 0x29, // LoadDigitSpriteToPointer v0=0
    0xD1, 0x25, // Draw a 5 byte sprite at v1=0, v2=C
    // 9
    0x60, 0x09, // Load register v0=9
    0x61, 0x05, // Load register v1=5
    0x62, 0x0C, // Load register v2=C
    0xF0, 0x29, // LoadDigitSpriteToPointer v0=1
    0xD1, 0x25, // Draw a 5 byte sprite at v1=5, v2=C
    // A
    0x60, 0x0A, // Load register v0=A
    0x61, 0x0A, // Load register v1=10
    0x62, 0x0C, // Load register v2=C
    0xF0, 0x29, // LoadDigitSpriteToPointer v0=2
    0xD1, 0x25, // Draw a 5 byte sprite at v1=10, v2=C
    // B
    0x60, 0x0B, // Load register v0=B
    0x61, 0x0F, // Load register v1=15
    0x62, 0x0C, // Load register v2=C
    0xF0, 0x29, // LoadDigitSpriteToPointer v0=3
    0xD1, 0x25, // Draw a 5 byte sprite at v1=15, v2=C
    // C
    0x60, 0x0C, // Load register v0=C
    0x61, 0x00, // Load register v1=0
    0x62, 0x12, // Load register v2=0x12
    0xF0, 0x29, // LoadDigitSpriteToPointer v0=0
    0xD1, 0x25, // Draw a 5 byte sprite at v1=0, v2=0x12
    // D
    0x60, 0x0D, // Load register v0=D
    0x61, 0x05, // Load register v1=5
    0x62, 0x12, // Load register v2=0x12
    0xF0, 0x29, // LoadDigitSpriteToPointer v0=1
    0xD1, 0x25, // Draw a 5 byte sprite at v1=5, v2=0x12
    // E
    0x60, 0x0E, // Load register v0=E
    0x61, 0x0A, // Load register v1=10
    0x62, 0x12, // Load register v2=0x12
    0xF0, 0x29, // LoadDigitSpriteToPointer v0=2
    0xD1, 0x25, // Draw a 5 byte sprite at v1=10, v2=0x12
    // F
    0x60, 0x0F, // Load register v0=F
    0x61, 0x0F, // Load register v1=15
    0x62, 0x12, // Load register v2=0x12
    0xF0, 0x29, // LoadDigitSpriteToPointer v0=3
    0xD1, 0x25, // Draw a 5 byte sprite at v1=15, v2=0x12
    // Exit
    0x00, 0xFD, // Exit
];

static RENDER_DIGITS_DISPLAY: &str = "
XXXX...X..XXXX.XXXX.............................................
X..X..XX.....X....X.............................................
X..X...X..XXXX.XXXX.............................................
X..X...X..X.......X.............................................
XXXX..XXX.XXXX.XXXX.............................................
................................................................
X..X.XXXX.XXXX.XXXX.............................................
X..X.X....X.......X.............................................
XXXX.XXXX.XXXX...X..............................................
...X....X.X..X..X...............................................
...X.XXXX.XXXX..X...............................................
................................................................
XXXX.XXXX.XXXX.XXX..............................................
X..X.X..X.X..X.X..X.............................................
XXXX.XXXX.XXXX.XXX..............................................
X..X....X.X..X.X..X.............................................
XXXX.XXXX.X..X.XXX..............................................
................................................................
XXXX.XXX..XXXX.XXXX.............................................
X....X..X.X....X................................................
X....X..X.XXXX.XXXX.............................................
X....X..X.X....X................................................
XXXX.XXX..XXXX.X................................................
................................................................
................................................................
................................................................
................................................................
................................................................
................................................................
................................................................
................................................................
................................................................
";

#[test]
fn test_render_digits() {
    let logger = Logger::new(Box::new(|_| ()));
    let mut processor = Processor::new(logger);
    processor.initialize(&RENDER_DIGITS_PROGRAM);
    while !processor.run_next_instruction() {}
    let display = processor.get_display();
    let expected_display: Vec<&str> = RENDER_DIGITS_DISPLAY
        .trim()
        .split_ascii_whitespace()
        .collect();

    for i in 0..expected_display.len() {
        let row = expected_display[i];
        for j in 0..row.len() {
            let pixel = row.chars().nth(j).unwrap() != '.';
            assert_eq!(pixel, display.get_pixel(i, j));
        }
    }
}

