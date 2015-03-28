use std::collections::HashMap;

use super::challenge_2;
use util::Bytes;

pub fn decrypt(encrypted: &Bytes) -> Bytes {
    let key: u8 = find_encryption_key(encrypted);
    let repeated_key: Bytes = Bytes::from_repeating_byte(key, encrypted.len());

    challenge_2::xor(&encrypted, &repeated_key)
}

pub fn find_encryption_key(encrypted: &Bytes) -> u8 {
    let frequencies: HashMap<u8, u32> = compute_frequencies(&encrypted);

    let mut sorted_by_frequency: Bytes = encrypted.clone();
    sorted_by_frequency.sort();
    sorted_by_frequency.remove_duplicates();
    sorted_by_frequency.sort_by(|a, b| frequencies.get(b).unwrap().cmp(frequencies.get(a).unwrap()));

    let most_frequents_in_english: Vec<u8> = vec![69, 101, 84, 116, 65, 97, 79, 111, 73, 69, 78, 119]; // E, e, T, t, A, a, O, o, I, i, N, n

    let scores: HashMap<u8, u32> = build_scores();
    let mut best_key: u8 = 0;
    let mut best_score: u32 = 0;

    for encrypted_byte in sorted_by_frequency.iter() {
        for test_byte in most_frequents_in_english.iter() {
            let key = encrypted_byte ^ test_byte;
            let score = compute_key_score(key, &sorted_by_frequency, &scores);
            if score > best_score {
                best_score = score;
                best_key = key;
            }
        }
    }

    best_key
}

fn compute_key_score(key: u8, bytes: &Bytes, scores: &HashMap<u8, u32>) -> u32 {
    let mut score = 0;
    
    for byte in bytes.iter() {
        score += match scores.get(&(byte ^ key)) {
            Some(s) => *s,
            None => 0
        };
    }

    return score;
}

fn compute_frequencies(bytes: &Bytes) -> HashMap<u8, u32> {
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

pub fn build_scores() -> HashMap<u8, u32> {
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
