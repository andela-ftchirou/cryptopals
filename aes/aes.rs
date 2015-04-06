use util::Bytes;
use super::state::State;
use super::sbox::SBox;
use super::gfield;

pub struct AES;


impl AES {

    pub fn encrypt(input: &Bytes, key: &Bytes, init_vector: &Bytes) -> Bytes {
        let mut encrypted: Bytes = Bytes::new();
        let sbox: SBox = SBox::init();
        let key_schedule: Bytes = AES::expand_key(&key, &sbox);

        let prev_plain_block: &[u8] = init_vector;
        let prev_encrypted_block: &[u8; 16] = &[16; 0x00];

        let len = input.len();
        let mut start = 0;

        while start <= len - 2 {
            let mut block: &[u8] = &input[start..(start + 2)];
            //let mut processed_block: &[u8] = mode.process(block, prev_plain_block, prev_encrypted_block);
            let mut processed_block = block;

            match AES::cipher(processed_block, key_schedule, &sbox) {
                State(bytes) => {
                    encrypted.push(bytes);

                    start += 2;
                    prev_plain_block = block;
                    prev_encrypted_block = bytes;
                },

                _           => { panic!(""); }
            }
        }

        encrypted
    }

    fn cipher(byte: &[u8], key_schedule: &Bytes, rounds: usize, sbox: &SBox) -> State {
        let state: State = State(byte);

        AES::add_round_key(&state, &key_schedule[0..4]);

        for round in 1..(rounds + 1) {
            AES::sub_bytes(&state, &sbox);
            AES::shift_rows(&state);
            if round < rounds {
                AES::mix_columns(&state);
            }
            AES::add_round_key(&state, &key_schedule[(round * 4)..((round + 1) * 4)]);
        }

        state
    }

    fn add_round_key(state: &State, round_key: &[u8]) {
        let key = round_key[0];

        for column in 0..4 {
            let mut word: &[u8] = state.get_mut_word_at_column(column);
            
            for byte in word.iter_mut() {
                byte ^= key;
            }
        }
    }
    
    fn sub_bytes(state: &mut State, sbox: &SBox) {
        for row in 0..4 {
            for column in 0..4 {
                let byte = state.get(row, column);
                state.set(row, column, sbox.get(byte));
            }
        }
    }

    fn shift_rows(state: &mut State) {
        for row in 0..4 {
            for cycle in 0..row {
                for column in 1..4 {
                    let tmp = state.get(row - 1, column);
                    state.set(row - 1, column, state.get(row, column));
                    state.set(row, column, tmp);
                } 
            }
        }
    }

    fn mix_columns(state: &mut State) {
        let matrix: [u8; 16] = [0x02, 0x03, 0x01, 0x01,
                                0x01, 0x02, 0x03, 0x01,
                                0x01, 0x01, 0x02, 0x03,
                                0x03, 0x01, 0x01, 0x02];

        for column in 0..4 {
            let word: &mut [u8] = state.get_word_at_column(column);
            let mixed: &[u8; 4] = [4; 0x00];

            for row in 0..4 {
                let mixed_byte: u8 = 0;
                for i in 0..4 {
                    mixed_byte ^= gfield::mul(word[i], matrix[(row * 4) + column]);
                }

                state.set(row, column, mixed_byte);
            }
        }
    }

    pub fn expand_key(key: &Bytes, rounds: usize, sbox: &SBox) -> Bytes {
        let key_size: usize = key.len();
        let mut key_schedule: Bytes = Bytes::new();

        for byte in key.iter() {
            key_schedule.push(byte);
        }

        for round in key_size..(rounds + 1) {
            let previous_position = (round - 1) * 4;
            let position = round * 4;

            let previous_word: &[u8] = &key_schedule[previous_position..position];
            let mut temp: &[u8; 4] = [4; 0x00];

            if position % key_size == 0 {
                temp = AES::sub_word(AES::rot_word(previous_word), &sbox) ^ AES::rcon(round);
            } else if key_size > 6 && position % key_size == 4 {
                temp = AES::sub_word(previous_word, &sbox);
            }
            
            let earlier_word: &[u8] = &key_schedule[((round - key_size) * 4)..(((round - key_size) + 1) * 4)];

            for i in 0..4 {
                key_schedule.push(earlier_word[i] ^ temp[i]);
            }
        }

        key_schedule
    }

    fn sub_word(word: &[u8], sbox: &SBox) -> [u8; 4] {
        let mut output: [u8; 4] = [0x00; 4];

        for i in 0..4 {
            output[i] = sbox.get(word[i])
        }

        output
    }

    fn rot_word(word: &[u8]) -> &[u8] {
        let mut output: &[u8] = word.clone();

        for i in 1..4 {
            let tmp = output[i - 1];
            output[i - 1] = output[i];
            output[i] = tmp;
        }

        output
    }

    fn rcon(i: u8) -> [u8; 4] {
        let p: u8 = gfield::mul(0x02, i);
        let values: [u8; 4] = [p, 0x00, 0x00, 0x00];

        values
    }
}
