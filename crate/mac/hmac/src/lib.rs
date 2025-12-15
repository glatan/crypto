#![no_std]

extern crate alloc;

use alloc::{string::String, vec, vec::Vec};
use core::fmt::Write;

use cryptographic_hash_function::{BlockSize, CryptographicHashFunction, DigestSize};

pub trait Hmac {
    fn hmac_hash_to_bytes(&mut self, message: &[u8], key: &[u8]) -> Vec<u8>;
    fn hmac_hash_to_lower_hex(&mut self, message: &[u8], key: &[u8]) -> String {
        self.hmac_hash_to_bytes(message, key)
            .iter()
            .fold(String::new(), |mut hex, byte| {
                let _ = write!(hex, "{byte:02x}");
                hex
            })
    }
    fn hmac_hash_to_upper_hex(&mut self, message: &[u8], key: &[u8]) -> String {
        self.hmac_hash_to_bytes(message, key)
            .iter()
            .fold(String::new(), |mut hex, byte| {
                let _ = write!(hex, "{byte:02X}");
                hex
            })
    }
}

impl<T: CryptographicHashFunction + BlockSize + DigestSize + Clone> Hmac for T {
    fn hmac_hash_to_bytes(&mut self, message: &[u8], key: &[u8]) -> Vec<u8> {
        let mut inner_hasher = self.clone();
        let mut outer_hasher = self.clone();

        let mut zero_filled_key = vec![0u8; self.byte_block_len()];
        if key.len() > self.byte_block_len() {
            let hashed_key = self.hash_to_bytes(key);
            zero_filled_key[..self.byte_digest_len()].copy_from_slice(&hashed_key);
        } else {
            zero_filled_key[..key.len()].copy_from_slice(key);
        };

        let mut k_ipad = vec![0u8; self.byte_block_len() + message.len()];
        let mut k_opad = vec![0u8; self.byte_block_len() + self.byte_digest_len()];
        for i in 0..self.byte_block_len() {
            k_ipad[i] = zero_filled_key[i] ^ 0x36;
            k_opad[i] = zero_filled_key[i] ^ 0x5c;
        }
        k_ipad[self.byte_block_len()..].copy_from_slice(message);

        let hashed_k_ipad = inner_hasher.hash_to_bytes(&k_ipad);

        k_opad[self.byte_block_len()..].copy_from_slice(&hashed_k_ipad);

        outer_hasher.hash_to_bytes(&k_opad)
    }
}
