use std::fs::File;
use std::io::Write;

static KEY_PRESS_PROGRAM: [u8; 270] = [
/* Start*/         /* 0x200 */   0x00, 0xE0, // Clear the display
/* Digit 0 */      /* 0x202 */   0x60, 0x00, // Load register v0=0
                   /* 0x204 */   0xE0, 0x9E, // Skip if v0=0 pressed
                   /* 0x206 */   0x12, 0x12, // Jump to: Digit 1
                   /* 0x208 */   0x60, 0x00, // Load register v0=0
                   /* 0x20A */   0x61, 0x00, // Load register v1=0
                   /* 0x20C */   0x62, 0x00, // Load register v2=0
                   /* 0x20E */   0xF0, 0x29, // LoadDigitSpriteToPointer v0=0
                   /* 0x210 */   0xD1, 0x25, // Draw a 5 byte sprite at v1=0, v2=0
/* Digit 1 */      /* 0x212 */   0x60, 0x01, // Load register v0=1
                   /* 0x214 */   0xE0, 0x9E, // Skip if v0=1 pressed
                   /* 0x216 */   0x12, 0x22, // Jump to: Digit 2
                   /* 0x218 */   0x60, 0x01, // Load register v0=1
                   /* 0x21A */   0x61, 0x05, // Load register v1=5
                   /* 0x21C */   0x62, 0x00, // Load register v2=0
                   /* 0x21E */   0xF0, 0x29, // LoadDigitSpriteToPointer v0=1
                   /* 0x220 */   0xD1, 0x25, // Draw a 5 byte sprite at v1=5, v2=0
/* Digit 2 */      /* 0x222 */   0x60, 0x02, // Load register v0=2
                   /* 0x224 */   0xE0, 0x9E, // Skip if v0=2 pressed
                   /* 0x226 */   0x12, 0x32, // Jump to: Digit 3
                   /* 0x228 */   0x60, 0x02, // Load register v0=2
                   /* 0x22A */   0x61, 0x0A, // Load register v1=10
                   /* 0x22C */   0x62, 0x00, // Load register v2=0
                   /* 0x22E */   0xF0, 0x29, // LoadDigitSpriteToPointer v0=2
                   /* 0x230 */   0xD1, 0x25, // Draw a 5 byte sprite at v1=10, v2=0
/* Digit 3 */      /* 0x232 */   0x60, 0x03, // Load register v0=3
                   /* 0x234 */   0xE0, 0x9E, // Skip if v0=3 pressed
                   /* 0x236 */   0x12, 0x42, // Jump to: Digit 4
                   /* 0x238 */   0x60, 0x03, // Load register v0=3
                   /* 0x23A */   0x61, 0x0F, // Load register v1=15
                   /* 0x23C */   0x62, 0x00, // Load register v2=0
                   /* 0x23E */   0xF0, 0x29, // LoadDigitSpriteToPointer v0=3
                   /* 0x240 */   0xD1, 0x25, // Draw a 5 byte sprite at v1=15, v2=0
/* Digit 4 */      /* 0x242 */   0x60, 0x04, // Load register v0=4
                   /* 0x244 */   0xE0, 0x9E, // Skip if v0=4 pressed
                   /* 0x246 */   0x12, 0x52, // Jump to: Digit 5
                   /* 0x248 */   0x60, 0x04, // Load register v0=4
                   /* 0x24A */   0x61, 0x00, // Load register v1=0
                   /* 0x24C */   0x62, 0x06, // Load register v2=6
                   /* 0x24E */   0xF0, 0x29, // LoadDigitSpriteToPointer v0=0
                   /* 0x250 */   0xD1, 0x25, // Draw a 5 byte sprite at v1=0, v2=6
/* Digit 5 */      /* 0x252 */   0x60, 0x05, // Load register v0=5
                   /* 0x254 */   0xE0, 0x9E, // Skip if v0=5 pressed
                   /* 0x256 */   0x12, 0x62, // Jump to: Digit 6
                   /* 0x258 */   0x60, 0x05, // Load register v0=5
                   /* 0x25A */   0x61, 0x05, // Load register v1=5
                   /* 0x25C */   0x62, 0x06, // Load register v2=6
                   /* 0x25E */   0xF0, 0x29, // LoadDigitSpriteToPointer v0=1
                   /* 0x260 */   0xD1, 0x25, // Draw a 5 byte sprite at v1=5, v2=6
/* Digit 6 */      /* 0x262 */   0x60, 0x06, // Load register v0=6
                   /* 0x264 */   0xE0, 0x9E, // Skip if v0=6 pressed
                   /* 0x266 */   0x12, 0x72, // Jump to: Digit 7
                   /* 0x268 */   0x60, 0x06, // Load register v0=6
                   /* 0x26A */   0x61, 0x0A, // Load register v1=10
                   /* 0x26C */   0x62, 0x06, // Load register v2=6
                   /* 0x26E */   0xF0, 0x29, // LoadDigitSpriteToPointer v0=2
                   /* 0x270 */   0xD1, 0x25, // Draw a 5 byte sprite at v1=10, v2=6
/* Digit 7 */      /* 0x272 */   0x60, 0x07, // Load register v0=7
                   /* 0x274 */   0xE0, 0x9E, // Skip if v0=7 pressed
                   /* 0x276 */   0x12, 0x82, // Jump to: Digit 8
                   /* 0x278 */   0x60, 0x07, // Load register v0=7
                   /* 0x27A */   0x61, 0x0F, // Load register v1=15
                   /* 0x27C */   0x62, 0x06, // Load register v2=6
                   /* 0x27E */   0xF0, 0x29, // LoadDigitSpriteToPointer v0=3
                   /* 0x280 */   0xD1, 0x25, // Draw a 5 byte sprite at v1=15, v2=6
/* Digit 8 */      /* 0x282 */   0x60, 0x08, // Load register v0=8
                   /* 0x284 */   0xE0, 0x9E, // Skip if v0=8 pressed
                   /* 0x286 */   0x12, 0x92, // Jump to: Digit 9
                   /* 0x288 */   0x60, 0x08, // Load register v0=8
                   /* 0x28A */   0x61, 0x00, // Load register v1=0
                   /* 0x28C */   0x62, 0x0C, // Load register v2=C
                   /* 0x28E */   0xF0, 0x29, // LoadDigitSpriteToPointer v0=0
                   /* 0x290 */   0xD1, 0x25, // Draw a 5 byte sprite at v1=0, v2=C
/* Digit 9 */      /* 0x292 */   0x60, 0x09, // Load register v0=9
                   /* 0x294 */   0xE0, 0x9E, // Skip if v0=9 pressed
                   /* 0x296 */   0x12, 0xA2, // Jump to: Digit A
                   /* 0x298 */   0x60, 0x09, // Load register v0=9
                   /* 0x29A */   0x61, 0x05, // Load register v1=5
                   /* 0x29C */   0x62, 0x0C, // Load register v2=C
                   /* 0x29E */   0xF0, 0x29, // LoadDigitSpriteToPointer v0=1
                   /* 0x2A0 */   0xD1, 0x25, // Draw a 5 byte sprite at v1=5, v2=C
/* Digit A */      /* 0x2A2 */   0x60, 0x0A, // Load register v0=A
                   /* 0x2A4 */   0xE0, 0x9E, // Skip if v0=A pressed
                   /* 0x2A6 */   0x12, 0xB2, // Jump to: Digit B
                   /* 0x2A8 */   0x60, 0x0A, // Load register v0=A
                   /* 0x2AA */   0x61, 0x0A, // Load register v1=10
                   /* 0x2AC */   0x62, 0x0C, // Load register v2=C
                   /* 0x2AE */   0xF0, 0x29, // LoadDigitSpriteToPointer v0=2
                   /* 0x2B0 */   0xD1, 0x25, // Draw a 5 byte sprite at v1=10, v2=C
/* Digit B */      /* 0x2B2 */   0x60, 0x0B, // Load register v0=B
                   /* 0x2B4 */   0xE0, 0x9E, // Skip if v0=B pressed
                   /* 0x2B6 */   0x12, 0xC2, // Jump to: Digit C
                   /* 0x2B8 */   0x60, 0x0B, // Load register v0=B
                   /* 0x2BA */   0x61, 0x0F, // Load register v1=15
                   /* 0x2BC */   0x62, 0x0C, // Load register v2=C
                   /* 0x2BE */   0xF0, 0x29, // LoadDigitSpriteToPointer v0=3
                   /* 0x2C0 */   0xD1, 0x25, // Draw a 5 byte sprite at v1=15, v2=C
/* Digit C */      /* 0x2C2 */   0x60, 0x0C, // Load register v0=C
                   /* 0x2C4 */   0xE0, 0x9E, // Skip if v0=C pressed
                   /* 0x2C6 */   0x12, 0xD2, // Jump to: Digit D
                   /* 0x2C8 */   0x60, 0x0C, // Load register v0=C
                   /* 0x2CA */   0x61, 0x00, // Load register v1=0
                   /* 0x2CC */   0x62, 0x12, // Load register v2=0x12
                   /* 0x2CE */   0xF0, 0x29, // LoadDigitSpriteToPointer v0=0
                   /* 0x2D0 */   0xD1, 0x25, // Draw a 5 byte sprite at v1=0, v2=0x12
/* Digit D */      /* 0x2D2 */   0x60, 0x0D, // Load register v0=D
                   /* 0x2D4 */   0xE0, 0x9E, // Skip if v0=D pressed
                   /* 0x2D6 */   0x12, 0xE2, // Jump to: Digit E
                   /* 0x2D8 */   0x60, 0x0D, // Load register v0=D
                   /* 0x2DA */   0x61, 0x05, // Load register v1=5
                   /* 0x2DC */   0x62, 0x12, // Load register v2=0x12
                   /* 0x2DE */   0xF0, 0x29, // LoadDigitSpriteToPointer v0=1
                   /* 0x2E0 */   0xD1, 0x25, // Draw a 5 byte sprite at v1=5, v2=0x12
/* Digit E */      /* 0x2E2 */   0x60, 0x0E, // Load register v0=E
                   /* 0x2E4 */   0xE0, 0x9E, // Skip if v0=E pressed
                   /* 0x2E6 */   0x12, 0xF2, // Jump to: Digit F
                   /* 0x2E8 */   0x60, 0x0E, // Load register v0=E
                   /* 0x2EA */   0x61, 0x0A, // Load register v1=10
                   /* 0x2EC */   0x62, 0x12, // Load register v2=0x12
                   /* 0x2EE */   0xF0, 0x29, // LoadDigitSpriteToPointer v0=2
                   /* 0x2F0 */   0xD1, 0x25, // Draw a 5 byte sprite at v1=10, v2=0x12
/* Digit F */      /* 0x2F2 */   0x60, 0x0F, // Load register v0=F
                   /* 0x2F4 */   0xE0, 0x9E, // Skip if v0=F pressed
                   /* 0x2F6 */   0x13, 0x02, // Jump to: Delay
                   /* 0x2F8 */   0x60, 0x0F, // Load register v0=F
                   /* 0x2FA */   0x61, 0x0F, // Load register v1=15
                   /* 0x2FC */   0x62, 0x12, // Load register v2=0x12
                   /* 0x2FE */   0xF0, 0x29, // LoadDigitSpriteToPointer v0=3
                   /* 0x300 */   0xD1, 0x25, // Draw a 5 byte sprite at v1=15, v2=0x12
/* Delay */        /* 0x302 */   0x61, 0x3C, // Load 60 into v1
                   /* 0x304 */   0xF1, 0x15, // Load v1=60 into DT
/* Delay Check */  /* 0x306 */   0xF1, 0x07, // Load DT to v1
                   /* 0x308 */   0x41, 0x00, // Skip next if v1 != 0
                   /* 0x30A */   0x12, 0x00, // Jump to: Start
                   /* 0x30C */   0x13, 0x06, // Jump to: Delay check
];

fn main() {
    let mut file = File::create("key_press.ch8").expect("Failed to open output file");
    file.write_all(&KEY_PRESS_PROGRAM).expect("Failed to write to output file");
}
