mod chip8;
mod terminal_player;

fn main() {
    let chip8 = chip8::Chip8::new();
    let mut terminal_player = terminal_player::TerminalPlayer::new(chip8);
    let program = [
        0x62, 0x0F, // Load 0x0F to v2                             // 0x200
        0xF0, 0x29, // Load digit addr v0=0 into I                 // 0x202
        0x12, 0x0E, // Jump to: Draw                               // 0x204
        // Label: Undraw
        0xD8, 0x85, // Draw 5 byte sprite at (0,0) to erase        // 0x206
        0x70, 0x01, // Add 1 to v0                                 // 0x208
        0x80, 0x22, // And v0=v0&v2                                // 0x20A
        0xF0, 0x29, // Load digit addr v0=0 into I                 // 0x20C
        // Label: Draw
        0xD8, 0x85, // Draw 5 byte sprite at (0,0)                 // 0x20E
        0x61, 0x3C, // Load 60 into v1                             // 0x210
        0xF1, 0x15, // Load v1=60 into DT                          // 0x212
        // Label: DT check
        0xF1, 0x07, // Load DT to v1                               // 0x214
        0x41, 0x00, // Skip next if v1 != 0                        // 0x216
        0x12, 0x06, // Jump to: Undraw                             // 0x218
        0x12, 0x14, // Jump to: DT check                           // 0x21A
    ];
    terminal_player.run(&program);
}
