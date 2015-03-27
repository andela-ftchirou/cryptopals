use util::HexString;

pub fn xor(str1: &HexString, str2: &HexString) -> HexString {
    let bytes1: Vec<u8> = str1.to_bytes();
    let bytes2: Vec<u8> = str2.to_bytes();
    let mut result: Vec<u8> = Vec::new();
    let len = bytes1.len();

    for i in 0..len {
        result.push(bytes1[i] ^ bytes2[i]);
    }

    HexString::from_bytes(&result)
}
