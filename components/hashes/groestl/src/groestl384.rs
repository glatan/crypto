use alloc::vec::Vec;

use utils::Hash;

pub struct Groestl384;

impl Default for Groestl384 {
    fn default() -> Self {
        unimplemented!()
    }
}

impl Hash for Groestl384 {
    fn hash_to_bytes(&mut self, _message: &[u8]) -> Vec<u8> {
        unimplemented!()
    }
}
