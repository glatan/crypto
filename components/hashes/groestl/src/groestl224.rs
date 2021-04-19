use alloc::vec::Vec;

use utils::Hash;

pub struct Groestl224;

impl Default for Groestl224 {
    fn default() -> Self {
        unimplemented!()
    }
}

impl Hash for Groestl224 {
    fn hash_to_bytes(&mut self, _message: &[u8]) -> Vec<u8> {
        unimplemented!()
    }
}
