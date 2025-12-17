mod wasm_log_source;

use chip8::Chip8;
use chip8::logger::Logger;
use wasm_log_source::WasmLogSource;

use std::sync::OnceLock;
use wasm_bindgen::prelude::*;

static CHIP_8: OnceLock<Chip8> = OnceLock::new();

#[wasm_bindgen]
pub fn start_now() {
    let logger = Logger::new(Box::new(WasmLogSource::new()));
    let chip8 = Chip8::new(logger);
    CHIP_8.set(chip8).unwrap();
}
