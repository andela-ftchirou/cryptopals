use std::fs::File;
use std::io::Read;
use std::path::Path;

use super::challenge_1::Base64;
use super::challenge_2;
use super::challenge_3;
use super::challenge_4;
use super::challenge_5;
use super::challenge_6;
use super::challenge_7;

use util::Bytes;

#[test]
fn convert_hex_to_base64() {
    let bytes = Bytes::from_hex_string("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string());

    let base64 = Base64::encode(&bytes);

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

    assert_eq!("Cooking MC's like a pound of bacon", decrypted.to_ascii_string());
}

#[test]
fn detect_single_character_xor() {
    let path = Path::new("/home/faical/projects/cryptopals/src/set_1/challenge_4_data.txt");

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => panic!("{}", err)
    };

    let mut input = String::new();
    match file.read_to_string(&mut input) {
        Ok(_) => match challenge_4::detect_single_character_xor(input) {
            (encrypted, decrypted) => {
                assert_eq!("7b5a4215415d544115415d5015455447414c155c46155f4058455c5b523f", encrypted.to_hex_string());
                assert_eq!("Now that the party is jumping\n", decrypted.to_ascii_string());
            }
        },
        Err(err) => panic!("{}", err)
    }
}

#[test]
fn implement_repeating_key_xor() {
    let text = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal".to_string();

    let input: Bytes = Bytes::from_ascii_string(text);
    let key: Bytes = Bytes::from_ascii_string("ICE".to_string());

    let encrypted = challenge_5::encrypt_with_repeating_key_xor(&input, &key);

    assert_eq!("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f", encrypted.to_hex_string());
}

#[test]
fn break_repeating_key_xor() {
    let path = Path::new("/home/faical/projects/cryptopals/src/set_1/challenge_6_data.txt");

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => panic!("{}", err)
    };

    let mut input = String::new();
    match file.read_to_string(&mut input) {
        Err(err) => panic!("{}", err),
        Ok(_)    => {
            let base64: Base64 = Base64::from_string(input);

            let encrypted: Bytes = base64.decode();

            match challenge_6::break_repeating_key_xor(&encrypted) {
                (key, decrypted) => {
                    assert_eq!("Terminator X: Bring the noise", key.to_ascii_string());
                }
            }
        }
    }
}

#[test]
fn aes_in_ecb_mode() {
    let path = Path::new("/home/faical/projects/cryptopals/src/set_1/challenge_7_data.txt");

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => panic!("{}", err)
    };

    let mut input = String::new();
    match file.read_to_string(&mut input) {
        Err(err) => panic!("{}", err),
        Ok(_)    => {
            let base64: Base64 = Base64::from_string(input);

            let encrypted: Bytes = base64.decode();

            let decrypted: Bytes = challenge_7::decrypt_aes_in_ecb_mode(&encrypted);
            let decrypted_str = decrypted.to_ascii_string();

            assert_eq!("I'm back and I'm ringin' the bell", decrypted_str[0..33].to_string());
        }
    }
}
