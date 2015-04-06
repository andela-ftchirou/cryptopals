use util::Word;

pub struct State {
    storage: Vec<u8>
}

impl State {

    pub fn from_slice(slice: &[u8]) -> State {
        let mut storage: Vec<u8> = Vec::new();

        for byte in slice.iter() {
            storage.push(*byte);
        }

        State { storage: storage }
    }

    pub fn get_byte(&self, row: usize, column: usize) -> u8 {
        self.storage[row + (4 * column)]
    }

    pub fn get_word(&self, column: usize) -> Word {
        Word::from_slice(&self.storage[(column * 4)..((column + 1) * 4)])
    }

    pub fn set_byte(&mut self, row: usize, column: usize, byte: u8) {
        self.storage[row + (4 * column)] = byte
    }

    pub fn set_word(&mut self, column: usize, word: &Word) {
        let index = column * 4;
        for i in 0..4 {
            self.storage[i + index] = word[i];
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.storage.clone()
    }

    pub fn to_hex_string(&self) -> String {
        let table: [char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
                                 'a', 'b', 'c', 'd', 'e', 'f'];

        let mut str: String = String::new();

        for row in 0..4 {
            for column in 0..4 {
                let byte = self.get_byte(row, column);

                str.push(table[(byte >> 4) as usize]);
                str.push(table[(byte & 0x0f) as usize]);
                str.push(' ');
            }
            str.push('\n');
        }

        str
    }
}
