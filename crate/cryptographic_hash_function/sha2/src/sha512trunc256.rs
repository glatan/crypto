use alloc::vec::Vec;

use cryptographic_hash_function::{BlockSize, CryptographicHashFunction, DigestSize};

use super::{H512_TRUNC256, Sha2};

#[derive(Clone, Copy)]
pub struct Sha512Trunc256(Sha2<u64>);

impl Sha512Trunc256 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Sha512Trunc256 {
    #[rustfmt::skip]
    fn default() -> Self {
        Self(Sha2::<u64>::new(H512_TRUNC256))
    }
}

impl BlockSize for Sha512Trunc256 {
    fn byte_block_len(&self) -> usize {
        self.0.byte_block_len()
    }
}

impl DigestSize for Sha512Trunc256 {
    fn byte_digest_len(&self) -> usize {
        32
    }
}

impl CryptographicHashFunction for Sha512Trunc256 {
    fn hash_to_bytes(&mut self, message: &[u8]) -> Vec<u8> {
        self.0.sha2(message);
        self.0.status[0..4]
            .iter()
            .flat_map(|word| word.to_be_bytes().to_vec())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Sha512Trunc256;
    use dev_util::impl_test;

    const OFFICIAL: [(&[u8], &str); 2] = [
        // https://csrc.nist.gov/CSRC/media/Projects/Cryptographic-Standards-and-Guidelines/documents/examples/SHA512_256.pdf
        (
            "abc".as_bytes(),
            "53048e2681941ef99b2e29b76b4c7dabe4c2d0c634fc6d46e0e2f13107e7af23",
        ),
        (
            "abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu".as_bytes(),
            "3928e184fb8690f840da3988121d31be65cb9d3ef83ee6146feac861e19b563a",
        ),
    ];

    impl_test!(
        Sha512Trunc256,
        official,
        OFFICIAL,
        Sha512Trunc256::default()
    );
}
