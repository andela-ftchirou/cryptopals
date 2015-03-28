use util::Bytes;

pub fn encrypt_with_repeating_key_xor(input: &Bytes, key: &Bytes) -> Bytes {
    let mut encrypted: Bytes = Bytes::new();
    let input_len = input.len();
    let key_len = key.len();
    let mut start = 0;
    
    while start <= input_len - key_len {
        let bytes = &input[start..(start + key_len)];

        for i in 0..key_len {
            encrypted.push(bytes[i] ^ key[i]);
        }

        start += key_len;
    }

    let remaining = input_len - start;
    for i in 0..remaining {
        encrypted.push(input[start + i] ^ key[i]);
    }

    encrypted
}
    
