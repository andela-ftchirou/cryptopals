use super::challenge_1::Base64;
use super::challenge_2;

use util::HexString;

#[test]
fn convert_hex_to_base64() {
    let hex = HexString::from_string("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string());

    let base64 = Base64::from_bytes(&(hex.to_bytes()));

    assert_eq!("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t", base64.to_string());
}

#[test]
fn fixed_xor() {
    let hex1 = HexString::from_string("1c0111001f010100061a024b53535009181c".to_string());
    let hex2 = HexString::from_string("686974207468652062756c6c277320657965".to_string());
    
    let hex3 = challenge_2::xor(&hex1, &hex2);

    assert_eq!("746865206b696420646f6e277420706c6179", hex3.to_string());
}
