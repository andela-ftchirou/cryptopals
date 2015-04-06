use util::Bytes;
use util::Word;

use super::sbox::SBox;
use super::rcon::Rcon;
use super::state::State;
use super::gfield;
use super::blockprocessor::BlockProcessor;

pub struct AESEncryptor;

impl AESEncryptor {
    
    pub fn encrypt(input: &mut Bytes, key: &Bytes, block_processor: &BlockProcessor, init_vector: &Bytes) -> Bytes {
        let mut encrypted: Bytes = Bytes::new();
        let sbox: SBox = SBox::init();

        let rounds: usize = match key.len() {
            16 => 10,
            24 => 12,
            32 => 14,
            _  => panic!("invalid key length ({} bits), AES support only key of size 128 bits, 192 bits or 256 bits.", (key.len() * 8))
        };

        let key_schedule: Bytes = AESEncryptor::expand_key(&key, rounds, &sbox);

        while input.len() < 16 {
            input.push(0x00);
        }

        let len = input.len();
        let mut start = 0;

        while start <= len - 16 {
            let block: &[u8] = &input[start..(start + 16)];
            let processed_block: &[u8] = &block_processor.process(&block,
                                                                  if start > 0 { &input[(start - 16)..start] } else { &init_vector },
                                                                  if start > 0 { &encrypted[(start - 16)..start] } else { &init_vector });

            AESEncryptor::encrypt_block(block, &key_schedule, rounds, &sbox, &mut encrypted);

            start += 16;
        }

        if start < len {
            let mut remaining: Vec<u8> = input[start..len].to_vec();
            while remaining.len() < 16 {
                remaining.push(0x00);
            }

            let block: &[u8] = &block_processor.process(&remaining, &input[(start - 16)..start], &encrypted[(start - 16)..start]);
            AESEncryptor::encrypt_block(block, &key_schedule, rounds, &sbox, &mut encrypted);
        }

        encrypted
    }

    pub fn encrypt_block(block: &[u8], key_schedule: &Bytes, rounds: usize, sbox: &SBox, encrypted: &mut Bytes) {
        let output: State = AESEncryptor::cipher(&block, &key_schedule, rounds, &sbox);

        for column in 0..4 {
            for row in 0..4 {
                encrypted.push(output.get_byte(row, column));
            }
        }
    }

    pub fn cipher(byte: &[u8], key_schedule: &Bytes, rounds: usize, sbox: &SBox) -> State {
        let mut state: State = State::from_slice(byte);

        AESEncryptor::add_round_key(&mut state, &key_schedule[0..16]);

        for round in 1..(rounds + 1) {
            AESEncryptor::sub_bytes(&mut state, &sbox);
            AESEncryptor::shift_rows(&mut state);

            if round < rounds {
                AESEncryptor::mix_columns(&mut state);
            }

            AESEncryptor::add_round_key(&mut state, &key_schedule[(round * 16)..((round + 1) * 16)]);
        }

        state
    }

    pub fn add_round_key(state: &mut State, round_key: &[u8]) {
        let key: Word = Word::from_slice(round_key);

        for column in 0..4 {
            let state_word: Word = state.get_word(column);
            let key_word: Word = Word::from_slice(&round_key[(column * 4)..((column + 1) * 4)]);
            let mut word: Word = Word::new();

            for i in 0..4 {
                word[i] = state_word[i] ^ key_word[i];
            }

            state.set_word(column, &word);
        }
    }

    pub fn sub_bytes(state: &mut State, sbox: &SBox) {
        for row in 0..4 {
            for column in 0..4 {
                let byte = state.get_byte(row, column);
                state.set_byte(row, column, sbox.get(byte));
            }
        }
    }

    pub fn shift_rows(state: &mut State) {
        for row in 0..4 {
            for cycle in 0..row {
                for column in 1..4 {
                    let a = state.get_byte(row, column - 1);
                    let b = state.get_byte(row, column);

                    state.set_byte(row, column - 1, b);
                    state.set_byte(row, column, a);
                }
            }
        }
    }

    pub fn mix_columns(state: &mut State) {
        let matrix: [u8; 16] = [0x02, 0x03, 0x01, 0x01,
                                0x01, 0x02, 0x03, 0x01,
                                0x01, 0x01, 0x02, 0x03,
                                0x03, 0x01, 0x01, 0x02];

        for i in 0..4 {
            let word: Word = state.get_word(i);
            let mut mixed: Word = Word::new();

            for column in 0..4 {
                let mut byte: u8 = 0x00;

                for row in 0..4 {
                    byte ^= gfield::mul(word[row], matrix[row + (column * 4)]);
                }

                mixed[column] = byte;
            }

            state.set_word(i, &mixed);
        }
    }

    pub fn expand_key(key: &Bytes, rounds: usize, sbox: &SBox) -> Bytes {
        let nk: usize = key.len() / 4;

        let rcon: Rcon = Rcon::init();

        let mut key_schedule: Bytes = Bytes::new();

        for byte in key.iter() {
            key_schedule.push(*byte);
        }
        
        let mut i = nk;

        while i < 4 * (rounds + 1) {
            let prev_word_index = (i - 1) * 4;

            let mut temp: Word = Word::from_slice(&key_schedule[prev_word_index..(prev_word_index + 4)]);
            

            if i % nk == 0 {
                let rot: Word = AESEncryptor::rot_word(&temp);
                let sub: Word = AESEncryptor::sub_word(&rot, &sbox);
                let mut rcon_i = Word::new();
                rcon_i[0] = rcon[i / nk];

                temp = sub.xor(&rcon_i);

            } else if nk > 6 && i % nk == 4 {
                temp = AESEncryptor::sub_word(&temp, &sbox);
            }

            let n = (i - nk) * 4;
            let word: Word = temp.xor(&Word::from_slice(&key_schedule[n..(n + 4)]));
            for j in 0..4 {
                key_schedule.push(word[j]);
            }

            i += 1;
        }

        key_schedule
    }

    pub fn sub_word(word: &Word, sbox: &SBox) -> Word {
        let mut output: Word = Word::new();

        for i in 0..4 {
            output[i] = sbox.get(word[i]);
        }

        output
    }

    pub fn rot_word(word: &Word) -> Word {
        let mut output: Word = Word::new();

        for i in 1..4 {
            output[i - 1] = word[i];
        }

        output[3] = word[0];

        output
    }
}
