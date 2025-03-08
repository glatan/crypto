use alloc::vec::Vec;

use cryptographic_hash_function::CryptographicHashFunction;

pub struct Groestl512;

impl Default for Groestl512 {
    fn default() -> Self {
        unimplemented!()
    }
}

impl CryptographicHashFunction for Groestl512 {
    fn hash_to_bytes(&mut self, _message: &[u8]) -> Vec<u8> {
        unimplemented!()
    }
}
