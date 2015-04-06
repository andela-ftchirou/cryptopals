use std::collections::HashMap;

use util::Bytes;

use super::challenge_1::Base64;

pub fn detect_aes_in_ecb_mode(input: &String) -> String {
    for ciphertext in input.lines() {
        let mut block_frequency: HashMap<&str, u32> = HashMap::new();

        let len = ciphertext.len();
        let mut i = 0;

        while i < len - 32 {
            let block = &ciphertext[i..(i + 32)];

            if block_frequency.contains_key(block) {
                *(block_frequency.get_mut(block).unwrap()) += 1;
            } else {
                block_frequency.insert(block, 1);
            }

            i += 32;
        }

        for (block, frequency) in block_frequency.iter() {
            if *frequency > 1 {
                return ciphertext.to_string()
            }
        }
    }

    String::new()
}
