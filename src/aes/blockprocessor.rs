pub trait BlockProcessor {
    fn process(&self, block: &[u8], prev_plain_block: &[u8], prev_cipher_block: &[u8]) -> Vec<u8>;
}

pub struct ECB;

impl ECB {

    pub fn new() -> ECB {
        ECB
    }
}

pub struct CBC;

impl CBC {

    pub fn new() -> CBC {
        CBC
    }
}

impl BlockProcessor for ECB {

    fn process(&self, block: &[u8], prev_plain_block: &[u8], prev_cipher_block: &[u8]) -> Vec<u8> {
        block.to_vec()
    }
}

impl BlockProcessor for CBC {

    fn process(&self, block: &[u8], prev_plain_block: &[u8], prev_cipher_block: &[u8]) -> Vec<u8> {
        let mut processed: Vec<u8> = Vec::new();
        let len = block.len();

        for i in 0..len {
            processed.push(prev_cipher_block[i] ^ block[i]);
        }

        processed
    }
}
