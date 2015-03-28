use util::Bytes;

pub fn xor(a: &Bytes, b: &Bytes) -> Bytes {
    let mut result: Bytes = Bytes::new();
    let len = a.len();

    for i in 0..len {
        result.push(a[i] ^ b[i]);
    }

    result
}
