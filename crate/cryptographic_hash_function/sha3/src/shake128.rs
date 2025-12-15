use alloc::vec::Vec;

use cryptographic_hash_function::{BlockSize, CryptographicHashFunction, DigestSize};
use keccak::KeccakF1600;

#[derive(Clone, Copy)]
pub struct Shake128(KeccakF1600);

impl Shake128 {
    pub fn new(n: usize) -> Self {
        Self(KeccakF1600::new(1344, 256, n))
    }
}

impl Default for Shake128 {
    fn default() -> Self {
        Self::new(128 / 8)
    }
}

impl CryptographicHashFunction for Shake128 {
    fn hash_to_bytes(&mut self, message: &[u8]) -> Vec<u8> {
        self.0.keccak(message, 0x1F)
    }
}

impl BlockSize for Shake128 {
    fn byte_block_len(&self) -> usize {
        self.0.byte_block_len()
    }
}

impl DigestSize for Shake128 {
    fn byte_digest_len(&self) -> usize {
        self.0.byte_digest_len()
    }
}
