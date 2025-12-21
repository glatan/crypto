use alloc::vec::Vec;

use cryptographic_hash_function::{BlockSize, CryptographicHashFunction, DigestSize};
use keccak::KeccakF1600;

#[derive(Clone, Copy)]
pub struct Sha3_224(KeccakF1600);

impl Sha3_224 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Sha3_224 {
    fn default() -> Self {
        Self(KeccakF1600::new(1152, 448, 224 / 8))
    }
}

impl CryptographicHashFunction for Sha3_224 {
    fn hash_to_bytes(&mut self, message: &[u8]) -> Vec<u8> {
        // SHA-3のMbitsは01、つまり 01100000 → LSBファーストにする → 0b0000_0110 → 0x06
        // https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.202.pdf
        self.0.keccak(message, 0x06)
    }
}

impl BlockSize for Sha3_224 {
    fn byte_block_len(&self) -> usize {
        self.0.byte_block_len()
    }
}

impl DigestSize for Sha3_224 {
    fn byte_digest_len(&self) -> usize {
        self.0.byte_digest_len()
    }
}
