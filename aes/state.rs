struct State([u8]);

impl State {
    pub fn get(&self, row: u32, column: u32) -> u8 {
        match self {
            State(bytes) => bytes[row + (4 * column)],
            _            => panic!("invalid state.")
        }
    }

    pub fn get_word_at_column(&self, column: u32) -> &[u8] {
        match self {
            State(bytes) => &bytes[(column * 4)..((column + 1) * 4)],
            _            => panic!("invalid state.")
        }
    }

    pub fn get_mut_word_at_column(&self, column: u32) -> &mut [u8] {
        match self {
            State(bytes) => &bytes[(column * 4)..((column + 1) * 4)],
            _            => panic!("invalid state.")
        }
    }

    pub fn set(&mut self, row: u32, column: u32, byte: u8) {
        match self {
            State(bytes) => bytes[row + (4 * column)] = byte,
            _            => panic!("invalid state.")
        }
    }
}

        
