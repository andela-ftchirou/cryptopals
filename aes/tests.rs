use util::Bytes;
use super::aes;

#[test]
fn expand_128bit_key() {
    let key = Bytes::from_hex_string("2b7e151628aed2a6abf7158809cf4f3c");

    let key_schedule: Bytes = aes::expand_key(&key, 10);

    
}
