#![no_std]

extern crate alloc;

mod consts;
use alloc::vec::Vec;
use core::cmp::Ordering;

use cryptographic_hash_function::{BlockSize, CryptographicHashFunction, DigestSize};
use merkle_damgard::impl_md_flow;

use crate::consts::{IV, K};

#[inline(always)]
pub(crate) const fn ch(b: u32, c: u32, d: u32) -> u32 {
    (b & c) | (!b & d)
}
#[inline(always)]
pub(crate) const fn parity(b: u32, c: u32, d: u32) -> u32 {
    b ^ c ^ d
}
#[inline(always)]
pub(crate) const fn maj(b: u32, c: u32, d: u32) -> u32 {
    (b & c) | (b & d) | (c & d)
}

#[derive(Clone, Copy)]
pub struct Sha1 {
    status: [u32; 5],
}

impl Sha1 {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    #[allow(clippy::many_single_char_names, clippy::needless_range_loop)]
    fn compress(&mut self, m: &[u32; 16]) {
        let [mut a, mut b, mut c, mut d, mut e] = self.status;
        let mut temp;

        let mut w = [0; 80];
        w[..16].copy_from_slice(m);
        (16..80).for_each(|t| w[t] = (w[t - 3] ^ w[t - 8] ^ w[t - 14] ^ w[t - 16]).rotate_left(1));

        // Round 1
        for t in 0..20 {
            temp = a
                .rotate_left(5)
                .wrapping_add(ch(b, c, d))
                .wrapping_add(e)
                .wrapping_add(w[t])
                .wrapping_add(K[0]);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temp;
        }
        // Round 2
        for t in 20..40 {
            temp = a
                .rotate_left(5)
                .wrapping_add(parity(b, c, d))
                .wrapping_add(e)
                .wrapping_add(w[t])
                .wrapping_add(K[1]);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temp;
        }
        // Round 3
        for t in 40..60 {
            temp = a
                .rotate_left(5)
                .wrapping_add(maj(b, c, d))
                .wrapping_add(e)
                .wrapping_add(w[t])
                .wrapping_add(K[2]);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temp;
        }
        // Round 4
        for t in 60..80 {
            temp = a
                .rotate_left(5)
                .wrapping_add(parity(b, c, d))
                .wrapping_add(e)
                .wrapping_add(w[t])
                .wrapping_add(K[3]);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temp;
        }

        self.status[0] = self.status[0].wrapping_add(a);
        self.status[1] = self.status[1].wrapping_add(b);
        self.status[2] = self.status[2].wrapping_add(c);
        self.status[3] = self.status[3].wrapping_add(d);
        self.status[4] = self.status[4].wrapping_add(e);
    }
}

impl Default for Sha1 {
    #[rustfmt::skip]
    fn default() -> Self {
        Self {
            status: IV,
        }
    }
}

impl CryptographicHashFunction for Sha1 {
    fn hash_to_bytes(&mut self, message: &[u8]) -> Vec<u8> {
        impl_md_flow!(u32=> self, message, from_be_bytes, to_be_bytes);
        self.status
            .iter()
            .flat_map(|word| word.to_be_bytes().to_vec())
            .collect()
    }
}

impl BlockSize for Sha1 {
    fn byte_block_len(&self) -> usize {
        4 * 16
    }
}

impl DigestSize for Sha1 {
    fn byte_digest_len(&self) -> usize {
        20
    }
}

#[cfg(test)]
mod tests {
    use super::Sha1;
    use dev_util::impl_test;

    const OFFICIAL: [(&[u8], &str); 4] = [
        // https://tools.ietf.org/html/rfc3174
        ("abc".as_bytes(), "a9993e364706816aba3e25717850c26c9cd0d89d"),
        (
            "abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq".as_bytes(),
            "84983e441c3bd26ebaae4aa1f95129e5e54670f1",
        ),
        ("a".as_bytes(), "86f7e437faa5a7fce15d1ddcb9eaeaea377667b8"),
        (
            "0123456701234567012345670123456701234567012345670123456701234567".as_bytes(),
            "e0c094e867ef46c350ef54a7f59dd60bed92ae83",
        ),
    ];
    impl_test!(Sha1, official, OFFICIAL, Sha1::default());
}
