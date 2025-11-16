use fastrand;

#[derive(Debug)]
pub struct Chip8 {
    registers: Registers,
    memory: [u8; 4096],
    keys: u16,
    wait_for_keys: bool,
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

#[derive(Debug)]
enum Instruction {
    Unknown,
    ClearDisplay,
    Return,
    Jump { addr: u16 },
    Call { addr: u16 },
    SkipRegEqualsImm { x: u8, byte: u8 },
    SkipRegNotEqualsImm { x: u8, byte: u8 },
    SkipRegEqualsReg { x: u8, y: u8 },
    LoadImmToReg { x: u8, byte: u8 },
    AddImmToReg { x: u8, byte: u8 },
    LoadRegToReg { x: u8, y: u8 },
    OrReg { x: u8, y: u8 },
    AndReg { x: u8, y: u8 },
    XorReg { x: u8, y: u8 },
    AddReg { x: u8, y: u8 },
    SubReg { x: u8, y: u8 },
    ShiftRight { x: u8 },
    SubNegReg { x: u8, y: u8 },
    ShiftLeft { x: u8 },
    SkipRegNotEqualsReg { x: u8, y: u8 },
    LoadImmToPointer { addr: u16 },
    JumpOffset { addr: u16 },
    Random { x: u8, byte: u8 },
    Draw { x: u8, y: u8, nibble: u8 },
    SkipKeyPressed { x: u8 },
    SkipNotKeyPressed { x: u8 },
    LoadDelayTimerToReg { x: u8 },
    LoadNextKeyPress { x: u8 },
    LoadRegToDelayTimer { x: u8 },
    LoadRegToSoundTimer { x: u8 },
    AddRegToPointer { x: u8 },
    LoadDigitSpriteToPointer { x: u8 },
    LoadDecimalDigitsToPointer { x: u8 },
    WriteRegToPointer { x: u8 },
    ReadRegFromPointer { x: u8 },
}

fn get_bytes(word: u16) -> (u8, u8) {
    return (
        (word >> 8).try_into().unwrap(),
        (word & 0xFF).try_into().unwrap(),
    );
}

fn get_nibbles(byte: u8) -> (u8, u8) {
    return (byte >> 4, byte & 0xF);
}

/// Combine two bytes into a 12 bit address. This drops the high 4 bits of byte1.
fn make_addr(byte1: u8, byte2: u8) -> u16 {
    return ((byte1 & 0xF) as u16) << 8 | (byte2 as u16);
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
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
        }
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

    fn fetch_instruction(&mut self) -> Instruction {
        let byte1 = self.memory[self.registers.program_counter as usize];
        let byte2 = self.memory[(self.registers.program_counter + 1) as usize];
        self.registers.program_counter += 2;

        let addr = make_addr(byte1, byte2);
        let (nibble1, nibble2) = get_nibbles(byte1);
        let (nibble3, nibble4) = get_nibbles(byte2);

        match (nibble1, nibble2, nibble3, nibble4) {
            (0x0, 0x0, 0xE, 0x0) => Instruction::ClearDisplay,
            (0x0, 0x0, 0xE, 0xE) => Instruction::Return,
            (0x1, _, _, _) => Instruction::Jump { addr },
            (0x2, _, _, _) => Instruction::Call { addr },
            (0x3, _, _, _) => Instruction::SkipRegEqualsImm {
                x: nibble2,
                byte: byte2,
            },
            (0x4, _, _, _) => Instruction::SkipRegNotEqualsImm {
                x: nibble2,
                byte: byte2,
            },
            (0x5, _, _, 0x0) => Instruction::SkipRegEqualsReg {
                x: nibble2,
                y: nibble3,
            },
            (0x6, _, _, _) => Instruction::LoadImmToReg {
                x: nibble2,
                byte: byte2,
            },
            (0x7, _, _, _) => Instruction::AddImmToReg {
                x: nibble2,
                byte: byte2,
            },
            (0x8, _, _, 0x0) => Instruction::LoadRegToReg {
                x: nibble2,
                y: nibble3,
            },
            (0x8, _, _, 0x1) => Instruction::OrReg {
                x: nibble2,
                y: nibble3,
            },
            (0x8, _, _, 0x2) => Instruction::AndReg {
                x: nibble2,
                y: nibble3,
            },
            (0x8, _, _, 0x3) => Instruction::XorReg {
                x: nibble2,
                y: nibble3,
            },
            (0x8, _, _, 0x4) => Instruction::AddReg {
                x: nibble2,
                y: nibble3,
            },
            (0x8, _, _, 0x5) => Instruction::SubReg {
                x: nibble2,
                y: nibble3,
            },
            (0x8, _, _, 0x6) => Instruction::ShiftRight { x: nibble2 },
            (0x8, _, _, 0x7) => Instruction::SubNegReg {
                x: nibble2,
                y: nibble3,
            },
            (0x8, _, _, 0xE) => Instruction::ShiftLeft { x: nibble2 },
            (0x9, _, _, 0x0) => Instruction::SkipRegNotEqualsReg {
                x: nibble2,
                y: nibble3,
            },
            (0xA, _, _, _) => Instruction::LoadImmToPointer { addr },
            (0xB, _, _, _) => Instruction::JumpOffset { addr },
            (0xC, _, _, _) => Instruction::Random {
                x: nibble2,
                byte: byte2,
            },
            (0xD, _, _, _) => Instruction::Draw {
                x: nibble2,
                y: nibble3,
                nibble: nibble4,
            },
            (0xE, _, 0x9, 0xE) => Instruction::SkipKeyPressed { x: nibble2 },
            (0xE, _, 0xA, 0x1) => Instruction::SkipNotKeyPressed { x: nibble2 },
            (0xF, _, 0x0, 0x7) => Instruction::LoadDelayTimerToReg { x: nibble2 },
            (0xF, _, 0x0, 0xA) => Instruction::LoadNextKeyPress { x: nibble2 },
            (0xF, _, 0x1, 0x5) => Instruction::LoadRegToDelayTimer { x: nibble2 },
            (0xF, _, 0x1, 0x8) => Instruction::LoadRegToSoundTimer { x: nibble2 },
            (0xF, _, 0x1, 0xE) => Instruction::AddRegToPointer { x: nibble2 },
            (0xF, _, 0x2, 0x9) => Instruction::LoadDigitSpriteToPointer { x: nibble2 },
            (0xF, _, 0x3, 0x3) => Instruction::LoadDecimalDigitsToPointer { x: nibble2 },
            (0xF, _, 0x5, 0x5) => Instruction::WriteRegToPointer { x: nibble2 },
            (0xF, _, 0x6, 0x5) => Instruction::ReadRegFromPointer { x: nibble2 },
            _ => Instruction::Unknown,
        }
    }

    fn execute_instruction(&mut self, instruction: Instruction) -> () {
        match instruction {
            Instruction::ClearDisplay => {
                for i in 0..Self::DISPLAY_BYTES {
                    self.memory[Self::DSPLAY_MEM_ADDR + i] = 0;
                }
            }
            Instruction::Return => {
                self.registers.program_counter = self.registers.stack_pointer;
                self.registers.stack_pointer -= 1;
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
                self.registers.general[x as usize] -= self.registers.general[y as usize];
            }
            Instruction::ShiftRight { x } => {
                self.registers.general[0xF] = self.registers.general[x as usize] & 0x1;
                self.registers.general[x as usize] >>= 1;
            }
            Instruction::SubNegReg { x, y } => {
                self.registers.general[0xF] =
                    if self.registers.general[y as usize] > self.registers.general[x as usize] {
                        1
                    } else {
                        0
                    };
                self.registers.general[x as usize] =
                    self.registers.general[y as usize] - self.registers.general[x as usize];
            }
            Instruction::ShiftLeft { x } => {
                self.registers.general[0xF] = if self.registers.general[x as usize] & 0x80 > 0 {
                    1
                } else {
                    0
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
                todo!();
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
                self.registers.pointer =
                    Self::DIGIT_SPRITES_MEM_ADDR + 5 * (self.registers.general[x as usize] as u16)
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
            Instruction::Unknown => todo!(),
        }
    }

    const DIGIT_SPRITES_MEM_ADDR: u16 = 0x0;
    const DISPLAY_BYTES: usize = (Self::DISPLAY_PIXELS_X * Self::DISPLAY_PIXELS_Y + 7) >> 3;
    const DISPLAY_PIXELS_X: usize = 64;
    const DISPLAY_PIXELS_Y: usize = 32;
    const DSPLAY_MEM_ADDR: usize = 0x0;
    const PROGRAM_MEM_ADDR: usize = 0x200;
}
