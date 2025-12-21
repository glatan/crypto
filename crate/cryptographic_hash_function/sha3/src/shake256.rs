use alloc::vec::Vec;

use cryptographic_hash_function::{BlockSize, CryptographicHashFunction, DigestSize};
use keccak::KeccakF1600;

#[derive(Clone, Copy)]
pub struct Shake256(KeccakF1600);

impl Shake256 {
    pub fn new(n: usize) -> Self {
        Self(KeccakF1600::new(1088, 512, n))
    }
}

impl Default for Shake256 {
    fn default() -> Self {
        Self::new(256 / 8)
    }
}

impl CryptographicHashFunction for Shake256 {
    fn hash_to_bytes(&mut self, message: &[u8]) -> Vec<u8> {
        // SHAKEのMbitsは1111、つまり 11111000 → LSBファーストにする → 0b00001_1111 → 0x1F
        // https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.202.pdf
        self.0.keccak(message, 0x1F)
    }
}

impl BlockSize for Shake256 {
    fn byte_block_len(&self) -> usize {
        self.0.byte_block_len()
    }
}

impl DigestSize for Shake256 {
    fn byte_digest_len(&self) -> usize {
        self.0.byte_digest_len()
    }
}
