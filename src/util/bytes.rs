use std::ops::Index;
use std::ops::IndexMut;
use std::ops::DerefMut;
use std::ops::Deref;
use std::ops::Range;
use std::ops::RangeTo;
use std::ops::RangeFrom;
use std::ops::RangeFull;

use super::funcs;

pub struct Bytes {
    raw: Vec<u8>
}

impl Bytes {
    pub fn new() -> Bytes {
        Bytes { raw: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.raw.len()
    }

    pub fn push(&mut self, byte: u8) {
        self.raw.push(byte)
    }

    pub fn from_raw_bytes(bytes: &Vec<u8>) -> Bytes {
        Bytes { raw: bytes.clone() }
    }

    pub fn to_raw_bytes(&self) -> Vec<u8> {
        self.raw.clone()
    }

    pub fn from_repeating_byte(byte: u8, n: usize) -> Bytes {
        let mut bytes: Vec<u8> = Vec::new();
        let mut repeat = n;

        while repeat > 0 {
            bytes.push(byte);
            repeat -= 1;
        }

        Bytes { raw: bytes }
    }

    pub fn from_hex_string(s: String) -> Bytes {
        let mut bytes: Vec<u8> = Vec::new();
        let len = s.len();
        let mut start = 0;

        while start <= len - 2 {
            let hex_chars = &s[start..(start + 2)];
            let mut digits: Vec<i32> = Vec::new();

            for c in hex_chars.chars() {
                digits.push(funcs::hex_char_to_digit(c))
            }

            let first = digits[0];
            let second = digits[1];

            if first >= 0 && second >= 0 {
                let byte = first << 4 | second;
                bytes.push(byte as u8);
            }

            start += 2;
        }

        Bytes { raw: bytes }
    }

    pub fn to_hex_string(&self) -> String {
        let table: [char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
                                  'a', 'b', 'c', 'd', 'e', 'f'];

        let mut str = String::new();
        for byte in self.raw.iter() {
            str.push(table[(*byte >> 4) as usize]);
            str.push(table[(*byte & 15) as usize]);
        }

        str
    }

    pub fn remove_duplicates(&mut self) {
        self.raw.dedup()
    }

    pub fn to_ascii_string(&self) -> String {
        let mut str = String::new();

        for byte in self.raw.iter() {
            str.push(*byte as char);
        }

        str
    }
}

impl Index<usize> for Bytes {
    type Output = u8;

    fn index(&self, index: &usize) -> &u8 {
        self.raw.index(index)
    }
}

impl Index<Range<usize>> for Bytes {
    type Output = [u8];

    fn index(&self, index: &Range<usize>) -> &[u8] {
        self.raw.index(index)
    }
}

impl Index<RangeTo<usize>> for Bytes {
    type Output = [u8];

    fn index(&self, index: &RangeTo<usize>) -> &[u8] {
        self.raw.index(index)
    }
}

impl Index<RangeFrom<usize>> for Bytes {
    type Output = [u8];

    fn index(&self, index: &RangeFrom<usize>) -> &[u8] {
        self.raw.index(index)
    }
}

impl Index<RangeFull> for Bytes {
    type Output = [u8];

    fn index(&self, index: &RangeFull) -> &[u8] {
        self.as_slice()
    }
}

impl IndexMut<usize> for Bytes {
    
    fn index_mut(&mut self, index: &usize) -> &mut u8 {
        self.raw.index_mut(index)
    }
}

impl Iterator for Bytes {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        match self.raw.iter().next() {
            Some(byte) => Some(*byte),
            None => None
        }
    }
}

impl Clone for Bytes {
    
    fn clone(&self) -> Bytes {
        Bytes { raw: self.raw.clone() }
    }

    fn clone_from(&mut self, source: &Bytes) {
        self.raw.clone_from(&(source.raw))
    }
}

impl AsSlice<u8> for Bytes {
    
    fn as_slice(&self) -> &[u8] {
        self.raw.as_slice()
    }
}

impl Deref for Bytes {
    type Target = [u8];
    
    fn deref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl DerefMut for Bytes {

    fn deref_mut(&mut self) -> &mut [u8] {
        self.raw.as_mut_slice()
    }
}
