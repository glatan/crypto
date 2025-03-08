use alloc::vec::Vec;

use cryptographic_hash_function::CryptographicHashFunction;

pub struct Groestl384;

impl Default for Groestl384 {
    fn default() -> Self {
        unimplemented!()
    }
}

impl CryptographicHashFunction for Groestl384 {
    fn hash_to_bytes(&mut self, _message: &[u8]) -> Vec<u8> {
        unimplemented!()
    }
}
