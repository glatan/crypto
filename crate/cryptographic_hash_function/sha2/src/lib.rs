#![no_std]

extern crate alloc;

mod consts;

mod sha224;
mod sha256;
mod sha384;
mod sha512;
mod sha512trunc224;
mod sha512trunc256;

use core::cmp::Ordering;

use cryptographic_hash_function::BlockSize;
use merkle_damgard::impl_md_flow;

use crate::consts::{H224, H256, H384, H512, H512_TRUNC224, H512_TRUNC256, K32, K64};

pub use sha224::Sha224;
pub use sha256::Sha256;
pub use sha384::Sha384;
pub use sha512::Sha512;
pub use sha512trunc224::Sha512Trunc224;
pub use sha512trunc256::Sha512Trunc256;

#[inline(always)]
pub(crate) const fn ch32(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (!x & z)
}
#[inline(always)]
pub(crate) const fn maj32(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (x & z) ^ (y & z)
}
#[inline(always)]
pub(crate) const fn big_sigma32_0(x: u32) -> u32 {
    x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22)
}
#[inline(always)]
pub(crate) const fn big_sigma32_1(x: u32) -> u32 {
    x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25)
}
#[inline(always)]
pub(crate) const fn small_sigma32_0(x: u32) -> u32 {
    x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3)
}
#[inline(always)]
pub(crate) const fn small_sigma32_1(x: u32) -> u32 {
    x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10)
}

#[inline(always)]
pub(crate) const fn ch64(x: u64, y: u64, z: u64) -> u64 {
    (x & y) ^ (!x & z)
}
#[inline(always)]
pub(crate) const fn maj64(x: u64, y: u64, z: u64) -> u64 {
    (x & y) ^ (x & z) ^ (y & z)
}
#[inline(always)]
pub(crate) const fn big_sigma64_0(x: u64) -> u64 {
    x.rotate_right(28) ^ x.rotate_right(34) ^ x.rotate_right(39)
}
#[inline(always)]
pub(crate) const fn big_sigma64_1(x: u64) -> u64 {
    x.rotate_right(14) ^ x.rotate_right(18) ^ x.rotate_right(41)
}
#[inline(always)]
pub(crate) const fn small_sigma64_0(x: u64) -> u64 {
    x.rotate_right(1) ^ x.rotate_right(8) ^ (x >> 7)
}
#[inline(always)]
pub(crate) const fn small_sigma64_1(x: u64) -> u64 {
    x.rotate_right(19) ^ x.rotate_right(61) ^ (x >> 6)
}

// Sha2<u32>: SHA-224 and SHA-256
// Sha2<u64>: SHA-384, SHA-512, SHA-512/224 and SHA-512/256
#[derive(Clone, Copy)]
struct Sha2<T> {
    status: [T; 8],
}

impl Sha2<u32> {
    const fn new(iv: [u32; 8]) -> Self {
        Self { status: iv }
    }
    #[inline(always)]
    #[allow(clippy::many_single_char_names, clippy::needless_range_loop)]
    fn compress(&mut self, m: &[u32; 16]) {
        let (mut temp_1, mut temp_2);
        // Initialize the eight working variables
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = self.status;
        // Prepare the message schedule
        let mut w = [0; 64];
        w[..16].copy_from_slice(m);
        for t in 16..64 {
            w[t] = small_sigma32_1(w[t - 2])
                .wrapping_add(w[t - 7])
                .wrapping_add(small_sigma32_0(w[t - 15]))
                .wrapping_add(w[t - 16]);
        }
        // Round
        for t in 0..64 {
            temp_1 = h
                .wrapping_add(big_sigma32_1(e))
                .wrapping_add(ch32(e, f, g))
                .wrapping_add(K32[t])
                .wrapping_add(w[t]);
            temp_2 = big_sigma32_0(a).wrapping_add(maj32(a, b, c));
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp_1);
            d = c;
            c = b;
            b = a;
            a = temp_1.wrapping_add(temp_2);
        }
        // Compute the intermediate hash value
        self.status[0] = self.status[0].wrapping_add(a);
        self.status[1] = self.status[1].wrapping_add(b);
        self.status[2] = self.status[2].wrapping_add(c);
        self.status[3] = self.status[3].wrapping_add(d);
        self.status[4] = self.status[4].wrapping_add(e);
        self.status[5] = self.status[5].wrapping_add(f);
        self.status[6] = self.status[6].wrapping_add(g);
        self.status[7] = self.status[7].wrapping_add(h);
    }
    fn sha2(&mut self, message: &[u8]) {
        impl_md_flow!(u32=> self, message, from_be_bytes, to_be_bytes);
    }
}

impl BlockSize for Sha2<u32> {
    fn byte_block_len(&self) -> usize {
        4 * 16
    }
}

impl Sha2<u64> {
    const fn new(iv: [u64; 8]) -> Self {
        Self { status: iv }
    }
    #[inline(always)]
    #[allow(clippy::many_single_char_names, clippy::needless_range_loop)]
    fn compress(&mut self, m: &[u64; 16]) {
        let (mut temp_1, mut temp_2);
        // Initialize the eight working variables
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = self.status;
        // Prepare the message schedule
        let mut w = [0; 80];
        w[..16].copy_from_slice(m);
        for t in 16..80 {
            w[t] = small_sigma64_1(w[t - 2])
                .wrapping_add(w[t - 7])
                .wrapping_add(small_sigma64_0(w[t - 15]))
                .wrapping_add(w[t - 16]);
        }
        // Round
        for t in 0..80 {
            temp_1 = h
                .wrapping_add(big_sigma64_1(e))
                .wrapping_add(ch64(e, f, g))
                .wrapping_add(K64[t])
                .wrapping_add(w[t]);
            temp_2 = big_sigma64_0(a).wrapping_add(maj64(a, b, c));
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp_1);
            d = c;
            c = b;
            b = a;
            a = temp_1.wrapping_add(temp_2);
        }
        // Compute the intermediate hash value
        self.status[0] = self.status[0].wrapping_add(a);
        self.status[1] = self.status[1].wrapping_add(b);
        self.status[2] = self.status[2].wrapping_add(c);
        self.status[3] = self.status[3].wrapping_add(d);
        self.status[4] = self.status[4].wrapping_add(e);
        self.status[5] = self.status[5].wrapping_add(f);
        self.status[6] = self.status[6].wrapping_add(g);
        self.status[7] = self.status[7].wrapping_add(h);
    }
    fn sha2(&mut self, message: &[u8]) {
        impl_md_flow!(u64=> self, message, from_be_bytes, to_be_bytes);
    }
}

impl BlockSize for Sha2<u64> {
    fn byte_block_len(&self) -> usize {
        8 * 16
    }
}
