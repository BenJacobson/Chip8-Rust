pub fn get_nibbles(byte: u8) -> (u8, u8) {
    return (byte >> 4, byte & 0xF);
}

/// Combine two bytes into a 12 bit address. This drops the high 4 bits of byte1.
pub fn make_addr(byte1: u8, byte2: u8) -> u16 {
    return ((byte1 & 0xF) as u16) << 8 | (byte2 as u16);
}

/// @return the (byte, bit) to index into display memory.
pub fn get_display_bit(x: u8, y: u8, row_size: usize) -> (usize, usize) {
    let bit = row_size * (y as usize) + (x as usize);
    let byte_index = bit >> 3;
    let bit_index = 7 - (bit & 0x7);
    return (byte_index, bit_index);
}
