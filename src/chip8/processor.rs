use super::bit_manipulation::*;
use super::display::*;
use super::instructions::*;
use crate::logger::Logger;

use fastrand;

#[derive(Debug)]
pub struct Processor {
    logger: Logger,
    registers: Registers,
    memory: [u8; 4096],
    keys: u16,
    wait_for_keys: bool,
    exit: bool,
}

#[derive(Debug)]
struct Registers {
    general: [u8; 16],
    pointer: u16,
    program_counter: u16,
    stack_pointer: u16,
    delay_timer: u8,
    sound_timer: u8,
}

const DIGIT_SPRITES: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

const DIGIT_SPRITES_MEM_ADDR: usize = 0x0;
const DISPLAY_BYTES: usize = (DISPLAY_PIXELS_X * DISPLAY_PIXELS_Y + 7) >> 3;
const DISPLAY_MEM_ADDR: usize = DIGIT_SPRITES.len();
const DISPLAY_PIXELS_X: usize = 64;
const DISPLAY_PIXELS_Y: usize = 32;
const PROGRAM_MEM_ADDR: usize = 0x200;

/// @return the (byte, bit) to index into display memory.
fn get_display_bit(x: u8, y: u8) -> (usize, usize) {
    let bit = DISPLAY_PIXELS_X * (y as usize) + (x as usize);
    let byte_index = bit >> 3;
    let bit_index = 7 - (bit & 0x7);
    return (byte_index, bit_index);
}

impl Processor {
    pub fn new(logger: Logger) -> Self {
        Self {
            logger,
            registers: Registers {
                general: [0; 16],
                pointer: 0,
                program_counter: 0,
                stack_pointer: 0,
                delay_timer: 0,
                sound_timer: 0,
            },
            memory: [0; 4096],
            keys: 0,
            wait_for_keys: false,
            exit: false,
        }
    }

    pub fn initialize(&mut self, program: &[u8]) {
        for i in 0..DIGIT_SPRITES.len() {
            self.memory[DIGIT_SPRITES_MEM_ADDR + i] = DIGIT_SPRITES[i];
        }
        for i in 0..program.len() {
            self.memory[PROGRAM_MEM_ADDR + i] = program[i];
        }
        self.registers.program_counter = PROGRAM_MEM_ADDR as u16;
    }

    pub fn run_next_instruction(&mut self) -> bool {
        if self.exit {
            return true;
        }

        let byte1 = self.memory[self.registers.program_counter as usize];
        let byte2 = self.memory[(self.registers.program_counter + 1) as usize];
        let instruction = decode_instruction(byte1, byte2);
        self.logger.log(
            format!(
                "0x{:x}: Executing instruction: {:?}",
                self.registers.program_counter, instruction
            )
            .as_str(),
        );
        self.registers.program_counter += 2;

        self.execute_instruction(instruction);
        return self.exit;
    }

    pub fn get_display<'a>(&'a self) -> Display<'a> {
        let data = &self.memory[DISPLAY_MEM_ADDR..DISPLAY_MEM_ADDR + DISPLAY_BYTES];
        Display::new(data, DISPLAY_PIXELS_X, DISPLAY_PIXELS_Y)
    }

    pub fn tick_timers(&mut self) -> () {
        if self.registers.delay_timer > 0 {
            self.registers.delay_timer -= 1;
        }
        if self.registers.sound_timer > 0 {
            self.registers.sound_timer -= 1;
        }
    }

    pub fn has_sound(&self) -> bool {
        return self.registers.sound_timer > 0;
    }

    pub fn set_keys(&mut self, keys: u16) -> () {
        self.keys = keys;
    }

    fn execute_instruction(&mut self, instruction: Instruction) -> () {
        match instruction {
            Instruction::ClearDisplay => {
                for i in 0..DISPLAY_BYTES {
                    self.memory[DISPLAY_MEM_ADDR + i] = 0;
                }
            }
            Instruction::Return => {
                self.registers.program_counter = self.registers.stack_pointer;
                self.registers.stack_pointer -= 1;
            }
            Instruction::Exit => {
                self.exit = true;
            }
            Instruction::Jump { addr } => {
                self.registers.program_counter = addr;
            }
            Instruction::Call { addr } => {
                self.registers.stack_pointer += 1;
                let (byte1, byte2) = get_bytes(self.registers.program_counter);
                self.memory[self.registers.stack_pointer as usize] = byte1;
                self.memory[self.registers.stack_pointer as usize] = byte2;
                self.registers.program_counter = addr;
            }
            Instruction::SkipRegEqualsImm { x, byte } => {
                if self.registers.general[x as usize] == byte {
                    self.registers.program_counter += 2;
                }
            }
            Instruction::SkipRegNotEqualsImm { x, byte } => {
                if self.registers.general[x as usize] != byte {
                    self.registers.program_counter += 2;
                }
            }
            Instruction::SkipRegEqualsReg { x, y } => {
                if self.registers.general[x as usize] == self.registers.general[y as usize] {
                    self.registers.program_counter += 2;
                }
            }
            Instruction::LoadImmToReg { x, byte } => {
                self.registers.general[x as usize] = byte;
            }
            Instruction::AddImmToReg { x, byte } => {
                self.registers.general[x as usize] += byte;
            }
            Instruction::LoadRegToReg { x, y } => {
                self.registers.general[x as usize] = self.registers.general[y as usize];
            }
            Instruction::OrReg { x, y } => {
                self.registers.general[x as usize] |= self.registers.general[y as usize];
            }
            Instruction::AndReg { x, y } => {
                self.registers.general[x as usize] &= self.registers.general[y as usize];
            }
            Instruction::XorReg { x, y } => {
                self.registers.general[x as usize] ^= self.registers.general[y as usize];
            }
            Instruction::AddReg { x, y } => {
                self.registers.general[x as usize] += self.registers.general[y as usize];
            }
            Instruction::SubReg { x, y } => {
                self.registers.general[0xF] =
                    match self.registers.general[x as usize] > self.registers.general[y as usize] {
                        true => 1,
                        false => 0,
                    };
                self.registers.general[x as usize] -= self.registers.general[y as usize];
            }
            Instruction::ShiftRight { x } => {
                self.registers.general[0xF] = self.registers.general[x as usize] & 0x1;
                self.registers.general[x as usize] >>= 1;
            }
            Instruction::SubNegReg { x, y } => {
                self.registers.general[0xF] =
                    match self.registers.general[y as usize] > self.registers.general[x as usize] {
                        true => 1,
                        false => 0,
                    };
                self.registers.general[x as usize] =
                    self.registers.general[y as usize] - self.registers.general[x as usize];
            }
            Instruction::ShiftLeft { x } => {
                self.registers.general[0xF] = match self.registers.general[x as usize] & 0x80 {
                    0 => 0,
                    _ => 1,
                };
                self.registers.general[x as usize] <<= 1;
            }
            Instruction::SkipRegNotEqualsReg { x, y } => {
                if self.registers.general[x as usize] != self.registers.general[y as usize] {
                    self.registers.program_counter += 2;
                }
            }
            Instruction::LoadImmToPointer { addr } => {
                self.registers.pointer = addr;
            }
            Instruction::JumpOffset { addr } => {
                self.registers.program_counter = (self.registers.general[0] as u16) + addr;
            }
            Instruction::Random { x, byte } => {
                self.registers.general[x as usize] = fastrand::u8(0..=u8::MAX) & byte;
            }
            Instruction::Draw { x, y, nibble } => {
                let mut erase = false;
                for i in 0..nibble {
                    for j in 0..8 {
                        let sprite_bit = (self.memory
                            [(self.registers.pointer as usize) + (i as usize)]
                            >> (7 - j))
                            & 1;
                        let col = self.registers.general[x as usize] + j;
                        let row = self.registers.general[y as usize] + i;
                        let (display_byte, display_bit) = get_display_bit(col, row);
                        let before = self.memory[DISPLAY_MEM_ADDR + display_byte];
                        self.memory[DISPLAY_MEM_ADDR + display_byte] ^= sprite_bit << display_bit;
                        let after = self.memory[DISPLAY_MEM_ADDR + display_byte];
                        if before > after {
                            erase = true;
                        }
                    }
                }
                self.registers.general[0xF] = if erase { 1 } else { 0 };
            }
            Instruction::SkipKeyPressed { x } => {
                if self.keys & (1 << self.registers.general[x as usize]) > 0 {
                    self.registers.program_counter += 2;
                }
            }
            Instruction::SkipNotKeyPressed { x } => {
                if self.keys & (1 << self.registers.general[x as usize]) == 0 {
                    self.registers.program_counter += 2;
                }
            }
            Instruction::LoadDelayTimerToReg { x } => {
                self.registers.general[x as usize] = self.registers.delay_timer;
            }
            Instruction::LoadNextKeyPress { x } => {
                self.wait_for_keys = true;
            }
            Instruction::LoadRegToDelayTimer { x } => {
                self.registers.delay_timer = self.registers.general[x as usize];
            }
            Instruction::LoadRegToSoundTimer { x } => {
                self.registers.sound_timer = self.registers.general[x as usize];
            }
            Instruction::AddRegToPointer { x } => {
                self.registers.pointer += self.registers.general[x as usize] as u16;
            }
            Instruction::LoadDigitSpriteToPointer { x } => {
                self.registers.pointer = (DIGIT_SPRITES_MEM_ADDR
                    + 5 * (self.registers.general[x as usize] as usize))
                    as u16;
            }
            Instruction::LoadDecimalDigitsToPointer { x } => {
                let ones = x % 10;
                let tens = (x / 10) % 10;
                let hundreds = (x / 100) % 10;

                self.memory[self.registers.pointer as usize] = hundreds;
                self.memory[(self.registers.pointer + 1) as usize] = tens;
                self.memory[(self.registers.pointer + 2) as usize] = ones;
            }
            Instruction::WriteRegToPointer { x } => {
                for i in 0..=(x as u16) {
                    self.memory[(self.registers.pointer + i) as usize] =
                        self.registers.general[i as usize];
                }
            }
            Instruction::ReadRegFromPointer { x } => {
                for i in 0..=(x as u16) {
                    self.registers.general[i as usize] =
                        self.memory[(self.registers.pointer + i) as usize];
                }
            }
            Instruction::Unknown { byte1, byte2 } => {
                panic!("Unknown instruction {:X} {:X}", byte1, byte2)
            }
        }
    }
}
