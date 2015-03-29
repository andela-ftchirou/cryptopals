use std::collections::HashMap;

use util::Bytes;
use util::funcs;

use super::challenge_2;
use super::challenge_3;
use super::challenge_4;
use super::challenge_5;

pub fn break_repeating_key_xor(encrypted: &Bytes) -> (Bytes, Bytes) {
    let key_sizes: Vec<usize> = guess_probable_key_sizes(&encrypted);
    let mut key: Bytes = Bytes::new();
    let mut decrypted: Bytes = Bytes::new();

    let scores: HashMap<u8, u32> = challenge_3::build_scores();
    let mut best_score = 0;

    for key_size in key_sizes {
        match decrypt(&encrypted, key_size) {
            (k, d) => {
                let score = challenge_4::compute_phrase_score(&d, &scores);
                if score > best_score {
                    best_score = score;
                    key = k;
                    decrypted = d;
                }
            }
        }
    }

    (key, decrypted)
}

pub fn decrypt(encrypted: &Bytes, key_size: usize) -> (Bytes, Bytes) {
    let mut key: Bytes = Bytes::new();
    let len = encrypted.len();

    for i in 0..key_size {
        let mut blocks: Bytes = Bytes::new();
        let mut start = 0;

        while start < len - key_size {
            let block = &encrypted[start..(start + key_size)];

            blocks.push(block[i]);

            start += key_size;
        }

        if start + i < len {
            blocks.push(encrypted[start + i]);
        }

        key.push(challenge_3::find_encryption_key(&blocks));
    }

    let repeated_key: Bytes = challenge_5::repeat_key(&key, len);
    let decrypted: Bytes = challenge_2::xor(&encrypted, &repeated_key);

    (key, decrypted)
}

pub fn guess_probable_key_sizes(encrypted: &Bytes) -> Vec<usize> {
    let mut guesses: Vec<usize> = Vec::new();
    let mut distances: [usize; 40] = [0; 40];
    let mut min_distance: usize = 100;

    for guess in 2..40 {
        let first = &encrypted[0..(guess * 2)];
        let second = &encrypted[(guess * 2)..(guess * 4)];

        let distance = hamming_distance(first, second) / guess;
        if distance < min_distance {
            min_distance = distance;
        }

        distances[guess] = distance;
    }

    for i in 0..40 {
        if distances[i] == min_distance {
            guesses.push(i);
        }
    }

    guesses
}

pub fn hamming_distance(a: &[u8], b: &[u8]) -> usize {
    let len = a.len();
    let mut distance: usize = 0;

    for i in 0..len {
        for j in 0..8 {
            if funcs::get_bit(j, a[i], 8) != funcs::get_bit(j, b[i], 8) {
                distance += 1;
            }
        }
    }

    distance
}

