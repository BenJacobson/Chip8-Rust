use chip8::logger::LogSource;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn console_log(message: &str);
}

#[derive(Debug)]
pub struct WasmLogSource {}

impl WasmLogSource {
    pub fn new() -> Self {
        WasmLogSource {}
    }
}

impl LogSource for WasmLogSource {
    fn write(&mut self, message: &str) {
        console_log(message);
    }
}
