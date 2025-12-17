use device_query::DeviceQuery;
use device_query::DeviceState;
use device_query::Keycode;
use itertools::Itertools;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use chip8::*;

pub struct TerminalPlayer {
    chip8: Chip8,
    device_state: DeviceState,
    prev_output: String,
}

const THROTTLE_MICROS: u32 = 10000;
const TICK_MICROS: u32 = 33333;

const KEY_MAP: [(Keycode, Chip8Key); 16] = [
    (Keycode::Key1, Chip8Key::_0),
    (Keycode::Key2, Chip8Key::_1),
    (Keycode::Key3, Chip8Key::_2),
    (Keycode::Key4, Chip8Key::_3),
    (Keycode::Q, Chip8Key::_4),
    (Keycode::W, Chip8Key::_5),
    (Keycode::E, Chip8Key::_6),
    (Keycode::R, Chip8Key::_7),
    (Keycode::A, Chip8Key::_8),
    (Keycode::S, Chip8Key::_9),
    (Keycode::D, Chip8Key::_A),
    (Keycode::F, Chip8Key::_B),
    (Keycode::Z, Chip8Key::_C),
    (Keycode::X, Chip8Key::_D),
    (Keycode::C, Chip8Key::_E),
    (Keycode::V, Chip8Key::_F),
];

impl TerminalPlayer {
    pub fn new(chip8: Chip8) -> Self {
        Self {
            chip8,
            device_state: DeviceState::new(),
            prev_output: String::new(),
        }
    }

    pub fn run(&mut self, program: &[u8]) {
        self.chip8.initialize(program);

        let mut last_frame = Instant::now();
        let mut last_throttle = Instant::now();
        while !self.chip8.run_next_instruction() {
            let elapsed_micros_throttle = last_throttle.elapsed().subsec_micros();
            if elapsed_micros_throttle >= THROTTLE_MICROS {
                // Yield thread and immeidately continue.
                thread::sleep(Duration::from_nanos(1));
                last_throttle += Duration::from_micros(THROTTLE_MICROS.into());
            }
            let elapsed_micros_frame = last_frame.elapsed().subsec_micros();
            if elapsed_micros_frame >= TICK_MICROS {
                self.chip8.tick_timers();
                self.chip8.set_keys(self.get_keys_pressed());
                self.print_display();
                last_frame += Duration::from_micros(TICK_MICROS.into());
            }
        }
        self.print_display();
    }

    fn print_display(&mut self) {
        let display = self.chip8.get_display();
        let output = (0..display.height)
            .map(|i| {
                (0..display.width)
                    .map(|j| display.get_pixel(i, j))
                    .map(|set| if set { '█' } else { '⠀' })
                    .collect::<String>()
            })
            .chain(["\n".to_string()])
            .join("\n");
        if output == self.prev_output {
            return;
        }
        println!("{}", output);
        self.prev_output = output;
    }

    fn get_keys_pressed(&self) -> Chip8Keys {
        let keys: Vec<Keycode> = self.device_state.get_keys();
        return KEY_MAP
            .iter()
            .filter(|(keycode, _)| keys.contains(keycode))
            .map(|(_, key)| *key as u16)
            .reduce(|acc, e| acc | e)
            .unwrap_or(0);
    }
}
