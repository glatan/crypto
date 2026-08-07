#![no_std]

extern crate alloc;

mod consts;

use alloc::vec::Vec;

use cryptographic_hash_function::{
    BlockSize, CryptographicHashFunction, DigestSize, buffer::Buffer,
};

use crate::consts::STABLE;

const BLOCK_SIZE: usize = 16;

#[derive(Clone, Copy)]
pub struct Md2 {
    buffer: Buffer<BLOCK_SIZE>,
    state: [u8; 48],
    checksum: [u8; BLOCK_SIZE],
}

impl Md2 {
    pub fn new() -> Self {
        Self::default()
    }
    fn padding(&mut self) {
        let padding_by = BLOCK_SIZE - self.buffer.len();
        self.buffer.fill_remaining(padding_by as u8);
    }
    #[allow(clippy::needless_range_loop)]
    // ChunksExactの長さがコンパイル時に特定できないので、blockの長さは指定していない
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
    // ChunksExactの長さがコンパイル時に特定できないので、blockの長さは指定していない
    #[allow(clippy::needless_range_loop)]
    fn update_checksum(&mut self, block: &[u8]) {
        let mut l = self.checksum[15];
        for i in 0..16 {
            let c = block[i];
            self.checksum[i] ^= STABLE[(c ^ l) as usize];
            l = self.checksum[i];
        }
    }
}

impl Default for Md2 {
    fn default() -> Self {
        Self {
            buffer: Buffer::new(),
            state: [0; 48],
            checksum: [0; 16],
        }
    }
}

impl CryptographicHashFunction for Md2 {
    fn update(&mut self, message: &[u8]) {
        let mut used_message_len = self.buffer.write(message);
        if self.buffer.is_full() {
            let block = *self.buffer.get_all();
            self.compress(&block);
            self.update_checksum(&block);
            self.buffer.clear();
        } else {
            return;
        }

        let message_len = message.len();
        while message_len - used_message_len >= BLOCK_SIZE {
            let block = &message[used_message_len..used_message_len + BLOCK_SIZE];
            self.compress(block);
            self.update_checksum(block);
            used_message_len += BLOCK_SIZE;
        }

        self.buffer.write(&message[used_message_len..]);
    }

    fn finalize(&mut self) -> Vec<u8> {
        self.padding();
        let final_block = *self.buffer.get_all();
        self.compress(&final_block);
        self.update_checksum(&final_block);

        let checksum_block = self.checksum;
        self.compress(&checksum_block);

        self.state[0..16].to_vec()
    }
}

impl BlockSize for Md2 {
    fn byte_block_len(&self) -> usize {
        16
    }
}

impl DigestSize for Md2 {
    fn byte_digest_len(&self) -> usize {
        16
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
