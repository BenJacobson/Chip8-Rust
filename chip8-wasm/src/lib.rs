#![feature(once_cell_get_mut)]

mod wasm_log_source;

use chip8::Chip8;
use chip8::Chip8Display;
use chip8::Chip8Keys;
use chip8::logger::Logger;
use wasm_log_source::WasmLogSource;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmDisplay {
    data: *const u8,
    width: usize,
    height: usize,
}

impl WasmDisplay {
    fn new(chip8_display: &Chip8Display) -> Self {
        WasmDisplay {
            data: chip8_display.data.as_ptr(),
            width: chip8_display.width,
            height: chip8_display.height,
        }
    }
}

#[wasm_bindgen]
impl WasmDisplay {
    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
        if !(x < self.width && y < self.height) {
            return false;
        }
        let (byte, bit) = get_display_bit(x as u8, y as u8, self.width);
        // SAFETY: The display data exists for the lifetime of the Chip8 instance. The JS
        // code is responsible for managing this lifetime.
        unsafe { *self.data.offset(byte as isize) & (1 << bit) != 0 }
    }
}

fn get_display_bit(x: u8, y: u8, row_size: usize) -> (usize, usize) {
    let bit = row_size * (y as usize) + (x as usize);
    let byte_index = bit >> 3;
    let bit_index = 7 - (bit & 0x7);
    return (byte_index, bit_index);
}

#[wasm_bindgen]
pub struct WasmChip8 {
    chip8: Chip8,
}

#[wasm_bindgen]
impl WasmChip8 {
    pub fn new() -> Self {
        let logger = Logger::new(Box::new(WasmLogSource::new()));
        let chip8 = Chip8::new(logger);
        WasmChip8 { chip8 }
    }

    pub fn load_program(&mut self, program: &[u8]) {
        self.chip8.initialize(program);
    }

    pub fn run_instructions(&mut self, num_instructions: u32) -> bool {
        for _ in 0..num_instructions {
            if !self.chip8.run_next_instruction() {
                return false;
            }
        }
        true
    }

    pub fn tick_timers(&mut self) {
        self.chip8.tick_timers();
    }

    pub fn set_keys(&mut self, keys: Chip8Keys) {
        self.chip8.set_keys(keys);
    }

    pub fn get_display(&self) -> WasmDisplay {
        let display = self.chip8.get_display();
        WasmDisplay::new(&display)
    }
}
