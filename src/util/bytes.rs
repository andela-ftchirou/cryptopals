use std::ops::Index;
use std::ops::IndexMut;
use std::ops::DerefMut;
use std::ops::Deref;
use std::ops::Range;
use std::ops::RangeTo;
use std::ops::RangeFrom;
use std::ops::RangeFull;

use super::funcs;

/// A wrapper around ```Vec<u8>``` to make it more convenient to work with a
/// stream of bytes.
///
/// # Examples
///
/// ```
/// let mut bytes = Bytes::new();
/// bytes.push(0x48);
/// bytes.push(0x65);
/// bytes.push(0x6c);
/// bytes.push(0x6c);
/// bytes.push(0x6f);
///
/// assert_eq!(bytes.to_hex_string(), "58656c6c6f");
/// assert_eq!(bytes.to_ascii_string(), "Hello".to_string());
/// ```
///
pub struct Bytes {
    raw: Vec<u8>
}

impl Bytes {
    /// Constructs a new and empty ```Bytes```.
    pub fn new() -> Bytes {
        Bytes { raw: Vec::new() }
    }

    /// Returns the number of byte in a ```Bytes```.
    pub fn len(&self) -> usize {
        self.raw.len()
    }

    /// Add a byte to the end of the ```Bytes```.
    pub fn push(&mut self, byte: u8) {
        self.raw.push(byte)
    }

    /// Constructs a new ```Bytes``` with the contents of
    /// the ```Vec<u8>``` passed as argument.
    pub fn from_raw_bytes(bytes: &Vec<u8>) -> Bytes {
        Bytes { raw: bytes.clone() }
    }

    /// Returns an equivalent ```Vec<u8>``` of a ```Bytes```.
    pub fn to_raw_bytes(&self) -> Vec<u8> {
        self.raw.clone()
    }

    /// Constructs a new ```Bytes``` by repeating ```byte```
    /// ```n``` times.
    ///
    /// Examples
    ///
    /// ```
    /// let bytes: Bytes = Bytes::from_repeating_key(0x41, 5);
    /// assert_eq!(bytes.to_ascii_string(), "AAAAA");
    ///
    pub fn from_repeating_byte(byte: u8, n: usize) -> Bytes {
        let mut bytes: Vec<u8> = Vec::new();
        let mut repeat = n;

        while repeat > 0 {
            bytes.push(byte);
            repeat -= 1;
        }

        Bytes { raw: bytes }
    }

    /// Constructs a new ```Bytes``` from a string of characters
    /// encoded with ASCII.
    ///
    /// Examples
    ///
    /// ```
    /// let bytes: Bytes = Bytes::from_ascii_string("Hello");
    /// assert_eq!(bytes.to_hex_string(), "58656c6c6f".to_string())
    ///
    pub fn from_ascii_string(s: String) -> Bytes {
        let mut bytes: Vec<u8> = Vec::new();

        for c in s.chars() {
            bytes.push(c as u8);
        }

        Bytes { raw: bytes }
    }

    /// Constructs a new ```Bytes``` from a hexadecimal string.
    ///
    /// Examples
    ///
    /// ```
    /// let bytes: Bytes = Bytes::from_hex_string("58656c6c6c");
    /// assert_eq!(bytes.to_ascii_string(), "Hello".to_string());
    ///
    pub fn from_hex_string(s: String) -> Bytes {
        let mut bytes: Vec<u8> = Vec::new();
        let len = s.len();
        let mut start = 0;

        // In a hexadecimal string, 2 characters represent a single byte.
        // The following while loop iterates over s by slice of 2
        // characters (hex_chars), constructs a byte with these 2 characters
        // and push it in the vector bytes.
        while start <= len - 2 {
            let hex_chars = &s[start..(start + 2)];
            let mut digits: Vec<i32> = Vec::new();

            for c in hex_chars.chars() {
                digits.push(funcs::hex_char_to_digit(c))
            }

            let first = digits[0];
            let second = digits[1];

            // To rebuild the byte from the 2 characters, the bits of the
            // first digit are shifted by 4 positions to the left and the
            // result is ORed with the second digit.
            // We don't loose any information by "the shift" operation because
            // since the digit only represents half of a byte, its 4 bits at the
            // left are all equal to 0.
            //
            //    first      = 0 0 0 0 0 1 0 0
            //
            //    first << 4 = 0 1 0 0 0 0 0 0
            //    second     = 0 0 0 0 1 0 0 0
            //                 --------------- (apply OR)
            //    byte       = 0 1 0 0 1 0 0 0
            //
            if first >= 0 && second >= 0 {
                let byte = first << 4 | second;
                bytes.push(byte as u8);
            }

            start += 2;
        }

        Bytes { raw: bytes }
    }

    /// Converts a ```Bytes``` to a string of ASCII encoded
    /// characters. This function does not check if the byte is
    /// a valid ASCII character.
    pub fn to_ascii_string(&self) -> String {
        let mut str = String::new();

        for byte in self.raw.iter() {
            str.push(*byte as char);
        }

        str
    }

    /// Converts a ```Bytes``` to a hexadecimal string.
    /// Each byte will correspond to 2 hexadecimal characters.
    pub fn to_hex_string(&self) -> String {
        let table: [char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
                                  'a', 'b', 'c', 'd', 'e', 'f'];

        let mut str = String::new();
        for byte in self.raw.iter() {
            // We need to split the byte into 2 parts. Each part
            // will then be converted into the corresponding hex character.
            // To get the first part, the byte is right-shifted by 4 positions.
            // To get the second part, the first 4 bits of the byte are
            // zero'ed by ANDing the byte with 0x0f.
            //
            //    byte      = 0 1 0 0 1 0 0 0 (0x41)
            //    byte >> 4 = 0 0 0 0 0 1 0 0 (4)
            //
            //    byte      = 0 1 0 0 1 0 0 0
            //                0 0 0 0 1 1 1 1 (0x0f)
            //         (AND)  ----------------
            //                0 0 0 0 1 0 0 0 (8)
            str.push(table[(*byte >> 4) as usize]);
            str.push(table[(*byte & 0x0f) as usize]);
        }

        str
    }

    /// Converts a ```Bytes``` to a binary string. Groups of 8 bits are
    /// separated by spaces in the string.
    pub fn to_binary_string(&self) -> String {
        let mut str = String::new();

        for byte in self.raw.iter() {
            for i in 0..8 {
                str.push(if funcs::get_bit(i, *byte, 8) == 0 { '0' } else { '1' });
            }
            str.push(' ');
        }

        str
    }

    /// Remotes duplicates in a sorted ```Bytes```.
    pub fn remove_duplicates(&mut self) {
        self.raw.dedup()
    }

    /// Pushes ```padding``` worth of 0x00000000 at the end of a ```Bytes```.
    pub fn pad_with_zero(&mut self, padding: usize) {
        let mut n = padding;

        while n > 0 {
            self.raw.push(0x00000000);
            n -= 1;
        }
    }
}

/// Implements various traits to make ```Bytes``` more convenient to use.
/// The following implementations only forward to the underlying ```Vec<u8>```.
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
