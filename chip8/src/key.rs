#[derive(Debug, Clone, Copy)]
pub enum Key {
    _0 = 1 << 0x0,
    _1 = 1 << 0x1,
    _2 = 1 << 0x2,
    _3 = 1 << 0x3,
    _4 = 1 << 0x4,
    _5 = 1 << 0x5,
    _6 = 1 << 0x6,
    _7 = 1 << 0x7,
    _8 = 1 << 0x8,
    _9 = 1 << 0x9,
    _A = 1 << 0xA,
    _B = 1 << 0xB,
    _C = 1 << 0xC,
    _D = 1 << 0xD,
    _E = 1 << 0xE,
    _F = 1 << 0xF,
}

pub type Keys = u16;

pub fn key_number_to_keys(x: u8) -> Keys {
    if x > 15 {
        0
    } else {
        1 << x
    }
}

pub fn keys_has_any_key(keys1: Keys, keys2: Keys) -> bool {
    return keys1 & keys2 != 0;
}

pub fn keys_to_key_number(keys: Keys) -> u8 {
    for i in 0..16 {
        if keys & (1 << i) != 0 {
            return i;
        }
    }
    0
}
