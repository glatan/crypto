use alloc::vec::Vec;

use cryptographic_hash_function::CryptographicHashFunction;
use keccak::KeccakF1600;

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
