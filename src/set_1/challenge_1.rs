use std::num::Int;

pub struct Base64 {
    storage: Vec<u8>,
    padding: u32
}

fn get_bit(n: u32, byte: u8) -> u8 {
    if byte & (1 << (7 - n)) == 0 { 0 } else { 1 }
}

fn compute_padding(bytes: &Vec<u8>) -> u32 {
    let mut len = bytes.len();
    let mut padding = 0;

    if len < 3 {
        padding = (3 - len) as u32;
    } else {
        while len % 3 != 0 {
            padding += 1;
            len += 1;
        }
    }

    padding
}

fn pad(bytes: &mut Vec<u8>, padding: u32) {
    let mut pad = padding;

    while pad > 0 {
        bytes.push(0);
        pad -= 1;
    }
}

impl Base64 {
    pub fn len(&self) -> usize {
        self.storage.len() - self.padding as usize
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.storage.clone()
    }

    pub fn from_bytes(bytes: &Vec<u8>) -> Base64 {
        let mut cbytes = bytes.clone();
        let mut base64: Vec<u8> = Vec::new();
        let padding = compute_padding(&cbytes);

        pad(&mut cbytes, padding);

        let len = cbytes.len();
        let groups = len / 3;
        
        for i in 0..groups {
            let start = i * 3;
            let group = &cbytes[start..(start + 3)];
            let mut bitstream: Vec<u8> = Vec::new();

            for byte in group.iter() {
                for i in 0..8 {
                    bitstream.push(get_bit(i, *byte));
                }
            }

            let mut left = 0;

            while left < 24 {
                let bits = &bitstream[left..(left + 6)];
                let mut base64_value = 0;
                let mut exp = 5;

                for bit in bits {
                    base64_value += bit * 2.pow(exp);
                    if exp > 0 { exp -= 1; }
                }

                base64.push(base64_value);

                left += 6;
            }
        }

        Base64 { storage: base64, padding: padding }
    }

    pub fn to_string(&self) -> String {
        let table: [char; 64] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
                             'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
                             'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
                             'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
                             '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];

        let mut base64_string = String::new();
        let len = self.len();
        let mut padding = self.padding;

        for i in 0..len {
            base64_string.push(table[self.storage[i] as usize]);
        }

        while padding > 0 {
            base64_string.push('=');
            padding -= 1;
        }
        
        base64_string
    }
}
