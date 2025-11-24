mod chip8;
mod terminal_player;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Expected command line argument of path to program.");
        return;
    }

    let chip8 = chip8::Chip8::new();
    let mut terminal_player = terminal_player::TerminalPlayer::new(chip8);
    let filepath = args[1].as_str();
    let program = fs::read(filepath).expect(format!("Failed to open file: {}", filepath).as_str());
    terminal_player.run(&program);
}
