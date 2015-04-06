use std::ops::Index;
use std::ops::IndexMut;

use util::funcs;

pub struct Word {
    storage: Vec<u8>
}

impl Word {

    pub fn new() -> Word {
        Word { storage: vec![0x00, 0x00, 0x00, 0x00] }
    }

    
    pub fn from_slice(slice: &[u8]) -> Word {
        let mut storage: Vec<u8> = Vec::new();

        for i in 0..4 {
            storage.push(slice[i]);
        }
        
        Word { storage: storage }
    }

    pub fn xor(&self, word: &Word) -> Word {
        let mut r: Word = Word::new();

        for i in 0..4  {
            r[i] = self[i] ^ word[i];
        }

        r
    }

    pub fn to_hex_string(&self) -> String {
        let table: [char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
                                 'a', 'b', 'c', 'd', 'e', 'f'];

        let mut str = String::new();
        for byte in self.storage.iter() {
            str.push(table[(*byte >> 4) as usize]);
            str.push(table[(*byte & 0x0f) as usize]);
        }
        
        str
    }

    pub fn to_binary_string(&self) -> String {
        let mut str = String::new();

        for byte in self.storage.iter() {
            for i in 0..8 {
                str.push(if funcs::get_bit(i, *byte, 8) == 0 { '0' } else { '1' });
            }
            str.push(' ');
        }

        str
    }

}

impl Iterator for Word {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        match self.storage.iter().next() {
            Some(byte) => Some(*byte),
            None       => None
        }
    }
}

impl Index<usize> for Word {
    type Output = u8;

    fn index(&self, index: &usize) -> &u8 {
        self.storage.index(index)
    }
}

impl IndexMut<usize> for Word {
    
    fn index_mut(&mut self, index: &usize) -> &mut u8 {
        self.storage.index_mut(index)
    }
}

impl AsSlice<u8> for Word {
    
    fn as_slice(&self) -> &[u8] {
        self.storage.as_slice()
    }
}
