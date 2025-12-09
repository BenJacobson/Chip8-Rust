pub fn get_nibbles(byte: u8) -> (u8, u8) {
    return (byte >> 4, byte & 0xF);
}

/// Combine two bytes into a 12 bit address. This drops the high 4 bits of byte1.
pub fn make_addr(byte1: u8, byte2: u8) -> u16 {
    return ((byte1 & 0xF) as u16) << 8 | (byte2 as u16);
}
