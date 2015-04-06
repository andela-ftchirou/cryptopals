use util::Bytes;

/// Pads `input` with PKCS#7 (http://tools.ietf.org/html/rfc5652#section-6.3)
/// so that its length will be a multiple of `block_size`.
/// In PKCS#7, inputs are padded with k - (l mod k) where
/// k is the block size, l is the length of the input.
pub fn pkcs7_pad(input: &mut Bytes, block_size: usize) {
    let len: usize = input.len();

    let n: usize = block_size - (len % block_size);

    let mut i = 0;
    while i < n {
        input.push(n as u8);
        i += 1;
    }
}
