use alloc::vec::Vec;

use cryptographic_hash_function::{BlockSize, CryptographicHashFunction, DigestSize};

use super::{EdonR, P256};

#[derive(Clone, Copy)]
pub struct EdonR256(EdonR<u32>);

impl EdonR256 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for EdonR256 {
    fn default() -> Self {
        Self(EdonR::<u32>::new(P256))
    }
}

impl BlockSize for EdonR256 {
    fn byte_block_len(&self) -> usize {
        self.0.byte_block_len()
    }
}

impl DigestSize for EdonR256 {
    fn byte_digest_len(&self) -> usize {
        32
    }
}

impl CryptographicHashFunction for EdonR256 {
    fn hash_to_bytes(&mut self, message: &[u8]) -> Vec<u8> {
        self.0.edonr(message);
        self.0.state[8..16]
            .iter()
            .flat_map(|word| word.to_le_bytes().to_vec())
            .collect()
    }
}
