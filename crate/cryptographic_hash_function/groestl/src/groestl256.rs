use alloc::vec::Vec;

use cryptographic_hash_function::CryptographicHashFunction;

pub struct Groestl256;

impl Default for Groestl256 {
    fn default() -> Self {
        unimplemented!()
    }
}

impl CryptographicHashFunction for Groestl256 {
    fn hash_to_bytes(&mut self, _message: &[u8]) -> Vec<u8> {
        unimplemented!()
    }
}
