use std::collections::HashMap;

use super::challenge_2;
use util::HexString;

pub fn decrypt(encrypted: &HexString) -> String {
    let key: u8 = find_encryption_key(encrypted);
    let repeated_key: HexString = HexString::from_repeating_byte(key, encrypted.len());

    let decrypted: HexString = challenge_2::xor(&encrypted, &repeated_key);
    
    decrypted.to_ascii_string()
}

pub fn find_encryption_key(encrypted: &HexString) -> u8 {
    let bytes: Vec<u8> = encrypted.to_bytes();
    let frequencies: HashMap<u8, u32> = compute_frequencies(&bytes);

    let mut encrypted_bytes: Vec<u8> = bytes.clone();
    encrypted_bytes.sort();
    encrypted_bytes.dedup();
    encrypted_bytes.sort_by(|a, b| frequencies.get(b).unwrap().cmp(frequencies.get(a).unwrap()));

    let most_frequents_in_english: Vec<u8> = vec![69, 101, 84, 116, 65, 97, 79, 111, 73, 69, 78, 119]; // E, e, T, t, A, a, O, o, I, i, N, n

    let scores: HashMap<u8, u32> = build_scores();
    let mut best_key: u8 = 0;
    let mut best_score: u32 = 0;

    for encrypted_byte in encrypted_bytes.iter() {
        for test_byte in most_frequents_in_english.iter() {
            let key = *encrypted_byte ^ *test_byte;
            let score = compute_key_score(key, &encrypted_bytes, &scores);
            if score > best_score {
                best_score = score;
                best_key = key;
            }
        }
    }

    best_key
}

fn compute_key_score(key: u8, bytes: &Vec<u8>, scores: &HashMap<u8, u32>) -> u32 {
    let mut score = 0;
    
    for byte in bytes.iter() {
        score += match scores.get(&(byte ^ key)) {
            Some(s) => *s,
            None => 0
        };
    }

    return score;
}

fn compute_frequencies(bytes: &Vec<u8>) -> HashMap<u8, u32> {
    let mut map: HashMap<u8, u32> = HashMap::new();

    for byte in bytes.iter() {
        if map.contains_key(byte) {
            *(map.get_mut(byte).unwrap()) += 1
        } else {
            map.insert(*byte, 1);
        }
    }

    map
}

fn build_scores() -> HashMap<u8, u32> {
    let letters = "ETA OINSHRDLCUMWFGYPBVKJXQZ";
    let mut scores: HashMap<u8, u32> = HashMap::new();
    let mut score = 27;

    for letter in letters.chars() {
        scores.insert(letter as u8, score);
        if letter != ' ' {
            scores.insert((letter as u8) + 32, score);
        }
        score -= 1;
    }

    scores
}
