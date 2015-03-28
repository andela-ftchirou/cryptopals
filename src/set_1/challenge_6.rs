use util::Bytes;
use util::funcs;

pub fn hamming_distance(a: &[u8], b: &[u8]) -> usize {
    let len = a.len();
    let mut distance: usize = 0;

    for i in 0..len {
        distance += hamming_distance_between_bytes(a[i], b[i]);
    }

    distance
}

pub fn hamming_distance_between_bytes(a: u8, b: u8) -> usize {
    let mut distance: usize = 0;

    for i in 0..8 {
        if funcs::get_bit(i, a, 8) != funcs::get_bit(i, b, 8) {
            distance += 1
        }
    }

    distance
}
