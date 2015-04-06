use util::Bytes;
use util::Word;

use super::sbox::SBox;
use super::sbox::InvSBox;
use super::rcon::Rcon;
use super::state::State;
use super::gfield;
use super::blockprocessor::BlockProcessor;
use super::aesencryptor::AESEncryptor;

pub struct AESDecryptor;

impl AESDecryptor {

    pub fn decrypt(cipher: &Bytes, key: &Bytes, block_processor: &BlockProcessor, init_vector: &Bytes) -> Bytes {
        let mut plain: Bytes = Bytes::new();
        let sbox: SBox = SBox::init();
        let inv_sbox: InvSBox = InvSBox::init();
        let key_size = key.len();

        let rounds: usize = match key_size {
            16 => 10,
            24 => 12,
            32 => 14,
            _  => panic!("invalid key length ({} bits), AES supports only key of size 128 bits, 192 bits or 256 bits.", (key_size * 8))
        };

        let key_schedule: Bytes = AESEncryptor::expand_key(&key, rounds, &sbox);

        let len = cipher.len();
        let mut start = 0;

        while start <= len - 16 {
            let block: &[u8] = &cipher[start..(start + 16)];

            let state: State = AESDecryptor::inv_cipher(block, &key_schedule, rounds, &inv_sbox);

            let output: Vec<u8> = block_processor.process(&(state.to_vec()),
                                                          if start > 0 { &plain[(start - 16)..start] } else { &init_vector },
                                                          if start > 0 { &cipher[(start - 16)..start] } else { &init_vector });
            for byte in output {
                plain.push(byte);
            }

            start += 16;
        }

        plain
    }

    pub fn inv_cipher(byte: &[u8], key_schedule: &Bytes, rounds: usize, inv_sbox: &InvSBox) -> State {
        let mut state: State = State::from_slice(byte);

        AESEncryptor::add_round_key(&mut state, &key_schedule[(rounds * 16)..((rounds + 1) * 16)]);
        
        let mut round = rounds - 1;

        while round >= 1 {
            AESDecryptor::inv_sub_bytes(&mut state, &inv_sbox);
            AESDecryptor::inv_shift_rows(&mut state);
            AESEncryptor::add_round_key(&mut state, &key_schedule[(round * 16)..((round + 1) * 16)]);
            AESDecryptor::inv_mix_columns(&mut state);

            round -= 1;
        }
        
        AESDecryptor::inv_sub_bytes(&mut state, &inv_sbox);
        AESDecryptor::inv_shift_rows(&mut state);
        AESEncryptor::add_round_key(&mut state, &key_schedule[0..16]);
        
        state
    }

    pub fn inv_shift_rows(state: &mut State) {
        for row in 0..4 {
            for cycle in 0..row {
                let mut column = 3;
                while column >= 1 {
                    let a = state.get_byte(row, column - 1);
                    let b = state.get_byte(row, column);

                    state.set_byte(row, column - 1, b);
                    state.set_byte(row, column, a);

                    column -= 1;
                }
            }
        }
    }

    pub fn inv_sub_bytes(state: &mut State, inv_sbox: &InvSBox) {
        for row in 0..4 {
            for column in 0..4 {
                let byte = state.get_byte(row, column);
                state.set_byte(row, column, inv_sbox.get(byte));
            }
        }
    }

    pub fn inv_mix_columns(state: &mut State) {
        let matrix: [u8; 16] = [0x0e, 0x0b, 0x0d, 0x09,
                                0x09, 0x0e, 0x0b, 0x0d,
                                0x0d, 0x09, 0x0e, 0x0b,
                                0x0b, 0x0d, 0x09, 0x0e];

        for i in 0..4 {
            let word: Word = state.get_word(i);
            let mut inv_mixed: Word = Word::new();

            for column in 0..4 {
                let mut byte: u8 = 0x00;

                for row in 0..4 {
                    byte ^= gfield::mul(word[row], matrix[row + (column * 4)]);
                }

                inv_mixed[column] = byte;
            }

            state.set_word(i, &inv_mixed);
        }
    }
    
}
