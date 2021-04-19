use alloc::vec::Vec;

use utils::Hash;

pub struct Groestl256;

impl Default for Groestl256 {
    fn default() -> Self {
        unimplemented!()
    }
}

impl Hash for Groestl256 {
    fn hash_to_bytes(&mut self, _message: &[u8]) -> Vec<u8> {
        unimplemented!()
    }
}
