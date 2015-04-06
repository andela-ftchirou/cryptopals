use util::Bytes;

use super::challenge_9;

#[test]
fn implement_pkcs7_padding() {
    let mut input: Bytes = Bytes::from_ascii_string("YELLOW SUBMARINE".to_string());

    challenge_9::pkcs7_pad(&mut input, 20);

    assert_eq!("59454c4c4f57205355424d4152494e4504040404", input.to_hex_string());
}
