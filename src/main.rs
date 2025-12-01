mod chip8;
mod logger;
mod terminal_player;

use logger::file_logger;
use logger::Logger;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Expected command line argument of path to program.");
        return;
    }

    let mut logger: Logger = file_logger::from_env_args();
    logger.log("Starting execution");
    let chip8 = chip8::Chip8::new(logger);
    let mut terminal_player = terminal_player::TerminalPlayer::new(chip8);
    let filepath = args[1].as_str();
    let program = fs::read(filepath).expect(format!("Failed to open file: {}", filepath).as_str());
    terminal_player.run(&program);
}
