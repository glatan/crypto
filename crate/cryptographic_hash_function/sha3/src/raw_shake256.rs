use alloc::vec::Vec;

use cryptographic_hash_function::{BlockSize, CryptographicHashFunction, DigestSize};
use keccak::KeccakF1600;

#[derive(Clone, Copy)]
pub struct RawShake256(KeccakF1600);

impl RawShake256 {
    pub fn new(n: usize) -> Self {
        Self(KeccakF1600::new(1088, 512, n))
    }
}

impl Default for RawShake256 {
    fn default() -> Self {
        Self::new(256 / 8)
    }
}

impl CryptographicHashFunction for RawShake256 {
    fn hash_to_bytes(&mut self, message: &[u8]) -> Vec<u8> {
        self.0.keccak(message, 0x07)
    }
}

impl BlockSize for RawShake256 {
    fn byte_block_len(&self) -> usize {
        self.0.byte_block_len()
    }
}

impl DigestSize for RawShake256 {
    fn byte_digest_len(&self) -> usize {
        self.0.byte_digest_len()
    }
}
