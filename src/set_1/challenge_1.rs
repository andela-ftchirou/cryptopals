use std::num::Int;

use util::Bytes;
use util::funcs;

pub struct Base64 {
    storage: Bytes,
    padding: usize
}

impl Base64 {

    pub fn encode(bytes: &Bytes) -> Base64 {
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
                    bitstream.push(funcs::get_bit(i, *byte, 8));
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

    pub fn decode(&self) -> Bytes {
        let mut decoded: Bytes = Bytes::new();
        let len = self.storage.len();
        let mut start = 0;

        while start <= len - 4 {
            let group = &(self.storage)[start..(start + 4)];
            let is_last_group: bool = (start == len - 4);
            let mut bitstream: Vec<u8> = Vec::new();

            for bits in group.iter() {
                for i in 0..6 {
                    bitstream.push(funcs::get_bit(i, *bits, 6));
                }
            }

            let mut left = 0;
            let bytes_size: usize = if !is_last_group { 24 } else { 24 - (self.padding * 8) };

            while left < bytes_size {
                let bits = &bitstream[left..(left + 8)];
                let mut byte = 0;
                let mut exp = 7;

                for bit in bits {
                    byte += bit * 2.pow(exp);
                    if exp > 0 { exp -= 1 }
                }

                decoded.push(byte);

                left += 8;
            }

            start += 4;
        }

        decoded
    }

    pub fn to_string(&self) -> String {
        let table: [char; 64] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
                             'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
                             'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
                             'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
                             '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];

        let mut base64_string = String::new();
        let len = self.storage.len() - self.padding;
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

fn compute_padding(bytes: &Bytes) -> usize {
    let mut len = bytes.len();
    let mut padding: usize = 0;

    if len < 3 {
        padding = (3 - len) as usize;
    } else {
        while len % 3 != 0 {
            padding += 1;
            len += 1;
        }
    }

    padding
}

fn pad(bytes: &mut Bytes, padding: usize) {
    let mut pad = padding;

    while pad > 0 {
        bytes.push(0);
        pad -= 1;
    }
}
