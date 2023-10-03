use alloc::vec::Vec;

use cryptographic_hash_function::CryptographicHashFunction;

use super::{Blake, IV256};

const ROUND: usize = 10;

pub struct Blake32(Blake<u32, ROUND>);

impl Blake32 {
    #[rustfmt::skip]
    pub const fn new(salt: [u32; 4]) -> Self {
        Self(Blake::<u32, ROUND>::new(IV256, salt))
    }
}

impl Default for Blake32 {
    #[rustfmt::skip]
    fn default() -> Self {
        Self(Blake::<u32, ROUND>::new(IV256, [0; 4]))
    }
}

impl CryptographicHashFunction for Blake32 {
    fn hash_to_bytes(&mut self, message: &[u8]) -> Vec<u8> {
        self.0.blake(message, 0x01);
        self.0
            .h
            .iter()
            .flat_map(|word| word.to_be_bytes().to_vec())
            .collect()
    }
}
