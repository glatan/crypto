use alloc::vec::Vec;

use cryptographic_hash_function::CryptographicHashFunction;

use super::{Blake, IV512};

const ROUND: usize = 14;

pub struct Blake64(Blake<u64, ROUND>);

impl Blake64 {
    #[rustfmt::skip]
    pub const fn new(salt: [u64; 4]) -> Self {
        Self(Blake::<u64, ROUND>::new(IV512, salt))
    }
}

impl Default for Blake64 {
    #[rustfmt::skip]
    fn default() -> Self {
        Self(Blake::<u64, ROUND>::new(IV512, [0; 4]))
    }
}

impl CryptographicHashFunction for Blake64 {
    fn hash_to_bytes(&mut self, message: &[u8]) -> Vec<u8> {
        self.0.blake(message, 0x01);
        self.0
            .h
            .iter()
            .flat_map(|word| word.to_be_bytes().to_vec())
            .collect()
    }
}
