use alloc::vec::Vec;

use cryptographic_hash_function::{BlockSize, CryptographicHashFunction, DigestSize};

use crate::KeccakF1600;

#[derive(Clone, Copy)]
pub struct Keccak224(KeccakF1600);

impl Keccak224 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Keccak224 {
    fn default() -> Self {
        Self(KeccakF1600::new(1152, 448, 224 / 8))
    }
}

impl CryptographicHashFunction for Keccak224 {
    fn hash_to_bytes(&mut self, message: &[u8]) -> Vec<u8> {
        self.0.keccak(message, 0x01)
    }
}

impl BlockSize for Keccak224 {
    fn byte_block_len(&self) -> usize {
        self.0.byte_block_len()
    }
}

impl DigestSize for Keccak224 {
    fn byte_digest_len(&self) -> usize {
        self.0.byte_digest_len()
    }
}
