use alloc::vec::Vec;

use cryptographic_hash_function::CryptographicHashFunction;

use super::{Blake, IV256};

const ROUND: usize = 14;

pub struct Blake256(Blake<u32, ROUND>);

impl Blake256 {
    #[rustfmt::skip]
    pub const fn new(salt: [u32; 4]) -> Self {
        Self(Blake::<u32 , ROUND>::new(IV256, salt))
    }
}

impl Default for Blake256 {
    #[rustfmt::skip]
    fn default() -> Self {
        Self(Blake::<u32 , ROUND>::new(IV256, [0; 4]))
    }
}

impl CryptographicHashFunction for Blake256 {
    fn hash_to_bytes(&mut self, message: &[u8]) -> Vec<u8> {
        self.0.blake(message, 0x01);
        self.0
            .h
            .iter()
            .flat_map(|word| word.to_be_bytes().to_vec())
            .collect()
    }
}
