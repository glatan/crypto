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
pub struct Sha0 {
    status: [u32; 5],
}

impl Sha0 {
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
        (16..80).for_each(|t| w[t] = w[t - 3] ^ w[t - 8] ^ w[t - 14] ^ w[t - 16]);

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

impl Default for Sha0 {
    #[rustfmt::skip]
    fn default() -> Self {
        Self {
            status: IV,
        }
    }
}

impl CryptographicHashFunction for Sha0 {
    fn hash_to_bytes(&mut self, message: &[u8]) -> Vec<u8> {
        impl_md_flow!(u32=> self, message, from_be_bytes, to_be_bytes);
        self.status
            .iter()
            .flat_map(|word| word.to_be_bytes().to_vec())
            .collect()
    }
}

impl BlockSize for Sha0 {
    fn byte_block_len(&self) -> usize {
        4 * 16
    }
}

impl DigestSize for Sha0 {
    fn byte_digest_len(&self) -> usize {
        20
    }
}

#[cfg(test)]
mod tests {
    use super::Sha0;
    use dev_util::impl_test;

    const OFFICIAL: [(&[u8], &str); 2] = [
        // https://web.archive.org/web/20180905102133/https://www-ljk.imag.fr/membres/Pierre.Karpman/fips180.pdf
        // https://crypto.stackexchange.com/questions/62055/where-can-i-find-a-description-of-the-sha-0-hash-algorithm/62071#62071
        ("abc".as_bytes(), "0164b8a914cd2a5e74c4f7ff082c4d97f1edf880"),
        (
            "abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq".as_bytes(),
            "d2516ee1acfa5baf33dfc1c471e438449ef134c8",
        ),
    ];
    impl_test!(Sha0, official, OFFICIAL, Sha0::default());
}
