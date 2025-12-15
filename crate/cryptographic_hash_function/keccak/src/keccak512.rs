use alloc::vec::Vec;

use cryptographic_hash_function::{BlockSize, CryptographicHashFunction, DigestSize};

use crate::KeccakF1600;

#[derive(Clone, Copy)]
pub struct Keccak512(KeccakF1600);

impl Keccak512 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Keccak512 {
    fn default() -> Self {
        Self(KeccakF1600::new(576, 1024, 512 / 8))
    }
}

impl CryptographicHashFunction for Keccak512 {
    fn hash_to_bytes(&mut self, message: &[u8]) -> Vec<u8> {
        self.0.keccak(message, 0x01)
    }
}

impl BlockSize for Keccak512 {
    fn byte_block_len(&self) -> usize {
        self.0.byte_block_len()
    }
}

impl DigestSize for Keccak512 {
    fn byte_digest_len(&self) -> usize {
        self.0.byte_digest_len()
    }
}
