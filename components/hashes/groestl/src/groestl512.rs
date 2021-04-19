use alloc::vec::Vec;

use utils::Hash;

pub struct Groestl512;

impl Default for Groestl512 {
    fn default() -> Self {
        unimplemented!()
    }
}

impl Hash for Groestl512 {
    fn hash_to_bytes(&mut self, _message: &[u8]) -> Vec<u8> {
        unimplemented!()
    }
}
