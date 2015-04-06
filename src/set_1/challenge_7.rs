use util::Bytes;

use aes::AESDecryptor;
use aes::ECB;

pub fn decrypt_aes_in_ecb_mode(encrypted: &Bytes) -> Bytes {
    let key: Bytes = Bytes::from_ascii_string("YELLOW SUBMARINE".to_string());
    let ecb: ECB = ECB::new();
    let init_vector: Bytes = Bytes::new();

    AESDecryptor::decrypt(encrypted, &key, &ecb, &init_vector)
}
