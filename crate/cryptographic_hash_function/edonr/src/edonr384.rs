use alloc::vec::Vec;

use cryptographic_hash_function::{BlockSize, CryptographicHashFunction, DigestSize};

use super::{EdonR, P384};

#[derive(Clone, Copy)]
pub struct EdonR384(EdonR<u64>);

impl EdonR384 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for EdonR384 {
    fn default() -> Self {
        Self(EdonR::<u64>::new(P384))
    }
}

impl BlockSize for EdonR384 {
    fn byte_block_len(&self) -> usize {
        self.0.byte_block_len()
    }
}

impl DigestSize for EdonR384 {
    fn byte_digest_len(&self) -> usize {
        48
    }
}

impl CryptographicHashFunction for EdonR384 {
    fn hash_to_bytes(&mut self, message: &[u8]) -> Vec<u8> {
        self.0.edonr(message);
        self.0.state[10..16]
            .iter()
            .flat_map(|word| word.to_le_bytes().to_vec())
            .collect()
    }
}
