use std::num::Int;

use util::Bytes;
use util::funcs;

pub struct Base64 {
    storage: Bytes,
    padding: u32
}

impl Base64 {
    pub fn len(&self) -> usize {
        self.storage.len() - self.padding as usize
    }

    pub fn to_bytes(&self) -> Bytes {
        self.storage.clone()
    }

    pub fn from_bytes(bytes: &Bytes) -> Base64 {
        let mut cbytes = bytes.clone();
        let mut base64: Bytes = Bytes::new();
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
                    bitstream.push(funcs::get_bit(i, *byte));
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

fn compute_padding(bytes: &Bytes) -> u32 {
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

fn pad(bytes: &mut Bytes, padding: u32) {
    let mut pad = padding;

    while pad > 0 {
        bytes.push(0);
        pad -= 1;
    }
}
