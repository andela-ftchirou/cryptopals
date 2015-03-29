use std::collections::HashMap;

use super::challenge_3;
use util::Bytes;

pub fn detect_single_character_xor(input: String) -> (Bytes, Bytes) {
    let mut single_character_xor: Bytes = Bytes::new();
    let mut single_character_xor_decrypted: Bytes = Bytes::new();

    let scores: HashMap<u8, u32> = challenge_3::build_scores();
    let mut best_score = 0;

    for line in input.lines() {
        let bytes: Bytes = Bytes::from_hex_string(line.to_string());
        
        let decrypted: Bytes = challenge_3::decrypt(&bytes);
        let score = compute_phrase_score(&bytes, &scores);
        
        if score > best_score {
            best_score = score;
            single_character_xor = bytes;
            single_character_xor_decrypted = decrypted;
        }
    }

    (single_character_xor, single_character_xor_decrypted)
}

pub fn compute_phrase_score(phrase: &Bytes, scores: &HashMap<u8, u32>) -> u32 {
    let mut score = 0;

    for byte in phrase.iter() {
        score += match scores.get(byte) {
            Some(s) => *s,
            None    => 0
        };
    }

    score
}
