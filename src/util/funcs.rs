pub fn get_bit(n: u32, byte: u8) -> u8 {
    if byte & (1 << (7 - n)) == 0 { 0 } else { 1 }
}
