pub fn get_bit(n: u32, word: u8, word_size: u32) -> u8 {
    if word & (1 << ((word_size - 1) - n)) == 0 { 0 } else { 1 }
}

pub fn hex_char_to_digit(c: char) -> i32 {
    match c.to_digit(16) {
        Some(d) => d as i32,
        None => -1
    }
}
