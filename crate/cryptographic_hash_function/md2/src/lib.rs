#![no_std]

extern crate alloc;

mod consts;

use alloc::vec::Vec;

use cryptographic_hash_function::CryptographicHashFunction;

use crate::consts::STABLE;

pub struct Md2 {
    state: [u8; 48],
}

impl Md2 {
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(clippy::needless_range_loop)]
    fn compress(&mut self, block: &[u8]) {
        self.state[16..32].copy_from_slice(block);
        for i in 0..16 {
            self.state[i + 32] = block[i] ^ self.state[i];
        }
        let mut t = 0;
        for i in 0..18 {
            for k in 0..48 {
                self.state[k] ^= STABLE[t];
                t = self.state[k] as usize;
            }
            t = (t + i) % 256;
        }
    }
    fn compress_checksum(&mut self, message: &[u8], padded_block: &[u8; 16]) {
        let mut checksum = [0u8; 16];
        let mut c;
        let mut l = 0;
        for i in 0..(message.len() / 16) {
            for j in 0..16 {
                c = message[16 * i + j];
                checksum[j] ^= STABLE[(c ^ l) as usize];
                l = checksum[j];
            }
        }
        for i in 0..16 {
            c = padded_block[i];
            checksum[i] ^= STABLE[(c ^ l) as usize];
            l = checksum[i];
        }
        self.compress(&checksum);
    }
}

impl Default for Md2 {
    fn default() -> Self {
        Self { state: [0; 48] }
    }
}

impl CryptographicHashFunction for Md2 {
    fn hash_to_bytes(&mut self, message: &[u8]) -> Vec<u8> {
        let len = message.len();
        if len == 0 {
            // First block is filled with 16 (padding bytes)
            self.compress(&[16; 16]);
            self.compress_checksum(&[], &[16; 16]);
        } else if len >= 16 {
            message
                .chunks_exact(16)
                .for_each(|block| self.compress(block));
        }
        if len != 0 {
            let paddlen = len % 16;
            let mut block = [(16 - paddlen) as u8; 16]; // padding
            let offset = len - paddlen;
            block[..paddlen].clone_from_slice(&message[offset..len]);
            self.compress(&block);
            self.compress_checksum(message, &block);
        }
        self.state[0..16].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::Md2;
    use dev_util::impl_test;

    const OFFICIAL: [(&[u8], &str); 6] = [
        // https://tools.ietf.org/html/rfc1319
        ("".as_bytes(), "8350e5a3e24c153df2275c9f80692773"),
        ("a".as_bytes(), "32ec01ec4a6dac72c0ab96fb34c0b5d1"),
        (
            "message digest".as_bytes(),
            "ab4f496bfb2a530b219ff33031fe06b0",
        ),
        (
            "abcdefghijklmnopqrstuvwxyz".as_bytes(),
            "4e8ddff3650292ab5a4108c3aa47940b",
        ),
        (
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".as_bytes(),
            "da33def2a42df13975352846c30338cd",
        ),
        (
            "12345678901234567890123456789012345678901234567890123456789012345678901234567890"
                .as_bytes(),
            "d5976f79d83d3a0dc9806c3c66f3efd8",
        ),
    ];
    impl_test!(Md2, md2_official, OFFICIAL, Md2::default());
}
