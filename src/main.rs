mod chip8;
mod terminal_player;

fn main() {
    let chip8 = chip8::Chip8::new();
    let mut terminal_player = terminal_player::TerminalPlayer::new(chip8);
    terminal_player.run();
}
