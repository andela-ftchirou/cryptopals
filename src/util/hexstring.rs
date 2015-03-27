pub struct HexString {
    storage: Vec<u8>
}

fn hex_char_to_digit(c: char) -> i32 {
    match c.to_digit(16) {
        Some(d) => d as i32,
        None => -1
    }
}

impl HexString {
    pub fn from_bytes(bytes: &Vec<u8>) -> HexString {
        HexString { storage: bytes.clone() }
    }

    pub fn from_string(s: String) -> HexString {
        let mut bytes: Vec<u8> = Vec::new();
        let len = s.len();
        let mut start = 0;

        while start <= len - 2 {
            let hex_chars = &s[start..(start + 2)];
            let mut digits: Vec<i32> = Vec::new();

            for c in hex_chars.chars() {
                digits.push(hex_char_to_digit(c))
            }

            let first = digits[0];
            let second = digits[1];

            if first >= 0 && second >= 0 {
                let byte = first << 4 | second;
                bytes.push(byte as u8);
            }

            start += 2;
        }

        HexString { storage: bytes }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.storage.clone()
    }

    pub fn to_string(&self) -> String {
        let table: [char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
                                  'a', 'b', 'c', 'd', 'e', 'f'];

        let mut str = String::new();
        for byte in self.storage.iter() {
            str.push(table[(*byte >> 4) as usize]);
            str.push(table[(*byte & 15) as usize]);
        }

        str
    }
}
