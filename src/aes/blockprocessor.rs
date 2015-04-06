pub trait BlockProcessor {
    fn process(&self, block: &[u8], prev_input_block: &[u8], prev_output_block: &[u8]) -> Vec<u8>;
}

pub struct ECB;

impl ECB {

    pub fn new() -> ECB {
        ECB
    }
}

impl BlockProcessor for ECB {

    fn process(&self, block: &[u8], prev_input_block: &[u8], prev_output_block: &[u8]) -> Vec<u8> {
        block.to_vec()
    }
}
