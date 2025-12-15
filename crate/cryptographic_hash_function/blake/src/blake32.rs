use alloc::vec::Vec;

use cryptographic_hash_function::{BlockSize, CryptographicHashFunction, DigestSize};

use super::{Blake, IV256};

const ROUND: usize = 10;

#[derive(Clone, Copy)]
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

impl BlockSize for Blake32 {
    fn byte_block_len(&self) -> usize {
        4 * 16
    }
}

impl DigestSize for Blake32 {
    fn byte_digest_len(&self) -> usize {
        32
    }
}
