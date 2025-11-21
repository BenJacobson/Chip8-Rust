mod chip8;
mod terminal_player;

fn main() {
    let chip8 = chip8::Chip8::new();
    let mut terminal_player = terminal_player::TerminalPlayer::new(chip8);
    let program: [u8; 2] = [0x00, 0xFD];
    terminal_player.run(&program);
}
