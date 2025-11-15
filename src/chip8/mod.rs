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
    LoadPointerImm { addr: u16 },
    JumpOffset { addr: u16 },
    Random { x: u8, byte: u8 },
    Draw { x: u8, y: u8, nibble: u8 },
    SkipKeyPressed { x: u8 },
    SkipNotKeyPressed { x: u8 },
    LoadFromDelayTimer { x: u8 },
    LoadNextKeyPress { x: u8 },
    LoadDelayTimer { x: u8 },
    LoadSoundTimer { x: u8 },
    AddRegToPointer { x: u8 },
    LoadPointerDigitSprite { x: u8 },
    LoadPointerDecimalDigits { x: u8 },
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
}
