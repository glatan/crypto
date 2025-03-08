use alloc::vec::Vec;

use cryptographic_hash_function::CryptographicHashFunction;

use super::{Blake, IV384};

const ROUND: usize = 14;

pub struct Blake48(Blake<u64, ROUND>);

impl Blake48 {
    #[rustfmt::skip]
    pub const fn new(salt: [u64; 4]) -> Self {
        Self(Blake::<u64, ROUND>::new(IV384, salt))
    }
}

impl Default for Blake48 {
    #[rustfmt::skip]
    fn default() -> Self {
        Self(Blake::<u64, ROUND>::new(IV384, [0; 4]))
    }
}

impl CryptographicHashFunction for Blake48 {
    fn hash_to_bytes(&mut self, message: &[u8]) -> Vec<u8> {
        self.0.blake(message, 0x00);
        self.0.h[0..6]
            .iter()
            .flat_map(|word| word.to_be_bytes().to_vec())
            .collect()
    }
}
