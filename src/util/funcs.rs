pub fn get_bit(n: u32, byte: u8) -> u8 {
    if byte & (1 << (7 - n)) == 0 { 0 } else { 1 }
}

pub fn hex_char_to_digit(c: char) -> i32 {
    match c.to_digit(16) {
        Some(d) => d as i32,
        None => -1
    }
}
