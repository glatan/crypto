// use alloc::vec::Vec;

use cryptographic_hash_function::CryptographicHashFunction;

use super::{Lake, IV256};

pub struct Lake256(Lake<u32>);

impl Lake256 {
    #[rustfmt::skip]
    pub fn new(salt: [u32; 4]) -> Self {
        Self(Lake::<u32>::new(IV256, salt))
    }
}

impl Default for Lake256 {
    #[rustfmt::skip]
    fn default() -> Self {
        Self(Lake::<u32>::new(IV256, [0; 4]))
    }
}

impl CryptographicHashFunction for Lake256 {
    fn hash_to_bytes(&mut self, message: &[u8]) -> Vec<u8> {
        self.0.lake(message);
        self.0
            .h
            .iter()
            .flat_map(|word| word.to_le_bytes().to_vec())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Lake256;
    use dev_util::impl_test;

    const OFFICIAL: [(&[u8], &str); 1] = [
        // https://link.springer.com/content/pdf/10.1007%2F978-3-540-71039-4_3.pdf
        (
            "".as_bytes(),
            "c5eb97ec704d48165a1714e3549343b418831b532fb84d85e304a0a473cb9e03",
        ),
    ];
    impl_test!(Lake256, official, OFFICIAL, Lake256::default());
}
