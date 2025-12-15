use alloc::vec::Vec;

use cryptographic_hash_function::{BlockSize, CryptographicHashFunction, DigestSize};
use keccak::KeccakF1600;

#[derive(Clone, Copy)]
pub struct Sha3_512(KeccakF1600);

impl Sha3_512 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Sha3_512 {
    fn default() -> Self {
        Self(KeccakF1600::new(576, 1024, 512 / 8))
    }
}

impl CryptographicHashFunction for Sha3_512 {
    fn hash_to_bytes(&mut self, message: &[u8]) -> Vec<u8> {
        self.0.keccak(message, 0x06)
    }
}

impl BlockSize for Sha3_512 {
    fn byte_block_len(&self) -> usize {
        self.0.byte_block_len()
    }
}

impl DigestSize for Sha3_512 {
    fn byte_digest_len(&self) -> usize {
        self.0.byte_digest_len()
    }
}
