#[derive(Debug)]
pub struct Chip8 {
    registers: Registers,
    memory: [u8; 4096],
    keys: u16,
}

#[derive(Debug)]
struct Registers {
    general: [u8; 16],
    pointer: u16,
    program_counter: u16,
    stack_pointer: u8,
    delay_timer: u8,
    sound_timer: u8,
}

#[derive(Debug)]
enum Instruction {
    Unknown,
    ClearScreen,
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

        let nibble1 = byte1 >> 4;
        let nibble2 = byte1 & 0xF;
        let nibble3 = byte2 >> 4;
        let nibble4 = byte2 & 0xF;
        let addr = (nibble2 as u16) << 8 | (byte2 as u16);

        match (nibble1, nibble2, nibble3, nibble4) {
            (0x0, 0x0, 0xE, 0x0) => Instruction::ClearScreen,
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
}
