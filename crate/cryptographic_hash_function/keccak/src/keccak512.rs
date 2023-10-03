use alloc::vec::Vec;

use cryptographic_hash_function::CryptographicHashFunction;

use crate::KeccakF1600;

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
