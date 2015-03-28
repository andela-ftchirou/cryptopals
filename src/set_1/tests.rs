use super::challenge_1::Base64;
use super::challenge_2;
use super::challenge_3;

use util::Bytes;

#[test]
fn convert_hex_to_base64() {
    let bytes = Bytes::from_hex_string("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string());

    let base64 = Base64::from_bytes(&bytes);

    assert_eq!("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t", base64.to_string());
}

#[test]
fn fixed_xor() {
    let a = Bytes::from_hex_string("1c0111001f010100061a024b53535009181c".to_string());
    let b = Bytes::from_hex_string("686974207468652062756c6c277320657965".to_string());
    
    let xor = challenge_2::xor(&a, &b);

    assert_eq!("746865206b696420646f6e277420706c6179", xor.to_hex_string());
}

#[test]
fn single_byte_xor_cipher() {
    let encrypted = Bytes::from_hex_string("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".to_string());
    
    let decrypted = challenge_3::decrypt(&encrypted);

    assert_eq!("Cooking MC's like a pound of bacon", decrypted);
}
