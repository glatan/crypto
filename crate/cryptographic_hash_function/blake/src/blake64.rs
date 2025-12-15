use alloc::vec::Vec;

use cryptographic_hash_function::{BlockSize, CryptographicHashFunction, DigestSize};

use super::{Blake, IV512};

const ROUND: usize = 14;

#[derive(Clone, Copy)]
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

impl BlockSize for Blake64 {
    fn byte_block_len(&self) -> usize {
        8 * 16
    }
}

impl DigestSize for Blake64 {
    fn byte_digest_len(&self) -> usize {
        64
    }
}
