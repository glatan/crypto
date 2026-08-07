#![no_std]

extern crate alloc;

pub mod buffer;

use alloc::{string::String, vec::Vec};
use core::fmt::Write;

pub trait CryptographicHashFunction {
    fn update(&mut self, message: &[u8]);
    fn finalize(&mut self) -> Vec<u8>;
    fn hash_to_bytes(&mut self, message: &[u8]) -> Vec<u8> {
        self.update(message);
        self.finalize()
    }
    fn hash_to_lowerhex(&mut self, message: &[u8]) -> String {
        self.hash_to_bytes(message)
            .iter()
            .fold(String::new(), |mut hex, byte| {
                let _ = write!(hex, "{byte:02x}");
                hex
            })
    }
    fn hash_to_upperhex(&mut self, message: &[u8]) -> String {
        self.hash_to_bytes(message)
            .iter()
            .fold(String::new(), |mut hex, byte| {
                let _ = write!(hex, "{byte:02X}");
                hex
            })
    }
}

pub trait BlockSize {
    fn bit_block_len(&self) -> usize {
        self.byte_block_len() * 8
    }
    fn byte_block_len(&self) -> usize;
}

pub trait DigestSize {
    fn bit_digest_len(&self) -> usize {
        self.byte_digest_len() * 8
    }
    fn byte_digest_len(&self) -> usize;
}

#[cfg(test)]
mod tests {
    use alloc::{vec, vec::Vec};

    use super::CryptographicHashFunction;

    struct Tester(Vec<u8>);
    impl Tester {
        fn new(bytes: Vec<u8>) -> Tester {
            Self(bytes)
        }
    }
    impl CryptographicHashFunction for Tester {
        fn update(&mut self, _: &[u8]) {}
        fn finalize(&mut self) -> Vec<u8> {
            self.0.clone()
        }
    }

    #[test]
    fn lower_hex() {
        assert_eq!(
            Tester::new(vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]).hash_to_lowerhex(&[]),
            "0123456789abcdef"
        );
        assert_eq!(Tester::new(vec![]).hash_to_lowerhex(&[]), "");
    }
    #[test]
    fn upper_hex() {
        assert_eq!(
            Tester::new(vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]).hash_to_upperhex(&[]),
            "0123456789ABCDEF"
        );
        assert_eq!(Tester::new(vec![]).hash_to_upperhex(&[]), "");
    }
}
