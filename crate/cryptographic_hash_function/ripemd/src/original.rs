// Integrity primitives for secure information systems : final report of RACE Integrity Primitives Evaluation RIPE RACE
// https://archive.org/details/isbn_9783540606406

use alloc::vec::Vec;
use core::cmp::Ordering;

use cryptographic_hash_function::{BlockSize, CryptographicHashFunction, DigestSize};
use merkle_damgard::impl_md_flow;

const fn f(x: u32, y: u32, z: u32) -> u32 {
    (x & y) | (!x & z)
}
const fn g(x: u32, y: u32, z: u32) -> u32 {
    (x & y) | (x & z) | (y & z)
}
const fn h(x: u32, y: u32, z: u32) -> u32 {
    x ^ y ^ z
}

const fn ff(a: u32, b: u32, c: u32, d: u32, x: u32, s: u32) -> u32 {
    a.wrapping_add(f(b, c, d)).wrapping_add(x).rotate_left(s)
}
const fn gg(a: u32, b: u32, c: u32, d: u32, x: u32, s: u32) -> u32 {
    a.wrapping_add(g(b, c, d))
        .wrapping_add(x)
        .wrapping_add(0x5A82_7999)
        .rotate_left(s)
}
const fn hh(a: u32, b: u32, c: u32, d: u32, x: u32, s: u32) -> u32 {
    a.wrapping_add(h(b, c, d))
        .wrapping_add(x)
        .wrapping_add(0x6ED9_EBA1)
        .rotate_left(s)
}

const fn fff(a: u32, b: u32, c: u32, d: u32, x: u32, s: u32) -> u32 {
    a.wrapping_add(f(b, c, d))
        .wrapping_add(x)
        .wrapping_add(0x50A2_8BE6)
        .rotate_left(s)
}

const fn ggg(a: u32, b: u32, c: u32, d: u32, x: u32, s: u32) -> u32 {
    a.wrapping_add(g(b, c, d)).wrapping_add(x).rotate_left(s)
}

const fn hhh(a: u32, b: u32, c: u32, d: u32, x: u32, s: u32) -> u32 {
    a.wrapping_add(h(b, c, d))
        .wrapping_add(x)
        .wrapping_add(0x5C4D_D124)
        .rotate_left(s)
}

#[derive(Clone, Copy)]
pub struct Ripemd {
    status: [u32; 4],
}

impl Ripemd {
    pub const fn new() -> Self {
        Self {
            status: [0x6745_2301, 0xEFCD_AB89, 0x98BA_DCFE, 0x1032_5476],
        }
    }
    #[inline(always)]
    const fn compress(&mut self, x: &[u32; 16]) {
        let [mut aa, mut bb, mut cc, mut dd] = self.status;
        let [mut aaa, mut bbb, mut ccc, mut ddd] = self.status;

        // round 1
        aa = ff(aa, bb, cc, dd, x[0], 11);
        dd = ff(dd, aa, bb, cc, x[1], 14);
        cc = ff(cc, dd, aa, bb, x[2], 15);
        bb = ff(bb, cc, dd, aa, x[3], 12);
        aa = ff(aa, bb, cc, dd, x[4], 5);
        dd = ff(dd, aa, bb, cc, x[5], 8);
        cc = ff(cc, dd, aa, bb, x[6], 7);
        bb = ff(bb, cc, dd, aa, x[7], 9);
        aa = ff(aa, bb, cc, dd, x[8], 11);
        dd = ff(dd, aa, bb, cc, x[9], 13);
        cc = ff(cc, dd, aa, bb, x[10], 14);
        bb = ff(bb, cc, dd, aa, x[11], 15);
        aa = ff(aa, bb, cc, dd, x[12], 6);
        dd = ff(dd, aa, bb, cc, x[13], 7);
        cc = ff(cc, dd, aa, bb, x[14], 9);
        bb = ff(bb, cc, dd, aa, x[15], 8);

        // round 2
        aa = gg(aa, bb, cc, dd, x[7], 7);
        dd = gg(dd, aa, bb, cc, x[4], 6);
        cc = gg(cc, dd, aa, bb, x[13], 8);
        bb = gg(bb, cc, dd, aa, x[1], 13);
        aa = gg(aa, bb, cc, dd, x[10], 11);
        dd = gg(dd, aa, bb, cc, x[6], 9);
        cc = gg(cc, dd, aa, bb, x[15], 7);
        bb = gg(bb, cc, dd, aa, x[3], 15);
        aa = gg(aa, bb, cc, dd, x[12], 7);
        dd = gg(dd, aa, bb, cc, x[0], 12);
        cc = gg(cc, dd, aa, bb, x[9], 15);
        bb = gg(bb, cc, dd, aa, x[5], 9);
        aa = gg(aa, bb, cc, dd, x[14], 7);
        dd = gg(dd, aa, bb, cc, x[2], 11);
        cc = gg(cc, dd, aa, bb, x[11], 13);
        bb = gg(bb, cc, dd, aa, x[8], 12);

        // round 3
        aa = hh(aa, bb, cc, dd, x[3], 11);
        dd = hh(dd, aa, bb, cc, x[10], 13);
        cc = hh(cc, dd, aa, bb, x[2], 14);
        bb = hh(bb, cc, dd, aa, x[4], 7);
        aa = hh(aa, bb, cc, dd, x[9], 14);
        dd = hh(dd, aa, bb, cc, x[15], 9);
        cc = hh(cc, dd, aa, bb, x[8], 13);
        bb = hh(bb, cc, dd, aa, x[1], 15);
        aa = hh(aa, bb, cc, dd, x[14], 6);
        dd = hh(dd, aa, bb, cc, x[7], 8);
        cc = hh(cc, dd, aa, bb, x[0], 13);
        bb = hh(bb, cc, dd, aa, x[6], 6);
        aa = hh(aa, bb, cc, dd, x[11], 12);
        dd = hh(dd, aa, bb, cc, x[13], 5);
        cc = hh(cc, dd, aa, bb, x[5], 7);
        bb = hh(bb, cc, dd, aa, x[12], 5);

        // parallel round 1
        aaa = fff(aaa, bbb, ccc, ddd, x[0], 11);
        ddd = fff(ddd, aaa, bbb, ccc, x[1], 14);
        ccc = fff(ccc, ddd, aaa, bbb, x[2], 15);
        bbb = fff(bbb, ccc, ddd, aaa, x[3], 12);
        aaa = fff(aaa, bbb, ccc, ddd, x[4], 5);
        ddd = fff(ddd, aaa, bbb, ccc, x[5], 8);
        ccc = fff(ccc, ddd, aaa, bbb, x[6], 7);
        bbb = fff(bbb, ccc, ddd, aaa, x[7], 9);
        aaa = fff(aaa, bbb, ccc, ddd, x[8], 11);
        ddd = fff(ddd, aaa, bbb, ccc, x[9], 13);
        ccc = fff(ccc, ddd, aaa, bbb, x[10], 14);
        bbb = fff(bbb, ccc, ddd, aaa, x[11], 15);
        aaa = fff(aaa, bbb, ccc, ddd, x[12], 6);
        ddd = fff(ddd, aaa, bbb, ccc, x[13], 7);
        ccc = fff(ccc, ddd, aaa, bbb, x[14], 9);
        bbb = fff(bbb, ccc, ddd, aaa, x[15], 8);

        // parallel round 2
        aaa = ggg(aaa, bbb, ccc, ddd, x[7], 7);
        ddd = ggg(ddd, aaa, bbb, ccc, x[4], 6);
        ccc = ggg(ccc, ddd, aaa, bbb, x[13], 8);
        bbb = ggg(bbb, ccc, ddd, aaa, x[1], 13);
        aaa = ggg(aaa, bbb, ccc, ddd, x[10], 11);
        ddd = ggg(ddd, aaa, bbb, ccc, x[6], 9);
        ccc = ggg(ccc, ddd, aaa, bbb, x[15], 7);
        bbb = ggg(bbb, ccc, ddd, aaa, x[3], 15);
        aaa = ggg(aaa, bbb, ccc, ddd, x[12], 7);
        ddd = ggg(ddd, aaa, bbb, ccc, x[0], 12);
        ccc = ggg(ccc, ddd, aaa, bbb, x[9], 15);
        bbb = ggg(bbb, ccc, ddd, aaa, x[5], 9);
        aaa = ggg(aaa, bbb, ccc, ddd, x[14], 7);
        ddd = ggg(ddd, aaa, bbb, ccc, x[2], 11);
        ccc = ggg(ccc, ddd, aaa, bbb, x[11], 13);
        bbb = ggg(bbb, ccc, ddd, aaa, x[8], 12);

        // parallel round 3
        aaa = hhh(aaa, bbb, ccc, ddd, x[3], 11);
        ddd = hhh(ddd, aaa, bbb, ccc, x[10], 13);
        ccc = hhh(ccc, ddd, aaa, bbb, x[2], 14);
        bbb = hhh(bbb, ccc, ddd, aaa, x[4], 7);
        aaa = hhh(aaa, bbb, ccc, ddd, x[9], 14);
        ddd = hhh(ddd, aaa, bbb, ccc, x[15], 9);
        ccc = hhh(ccc, ddd, aaa, bbb, x[8], 13);
        bbb = hhh(bbb, ccc, ddd, aaa, x[1], 15);
        aaa = hhh(aaa, bbb, ccc, ddd, x[14], 6);
        ddd = hhh(ddd, aaa, bbb, ccc, x[7], 8);
        ccc = hhh(ccc, ddd, aaa, bbb, x[0], 13);
        bbb = hhh(bbb, ccc, ddd, aaa, x[6], 6);
        aaa = hhh(aaa, bbb, ccc, ddd, x[11], 12);
        ddd = hhh(ddd, aaa, bbb, ccc, x[13], 5);
        ccc = hhh(ccc, ddd, aaa, bbb, x[5], 7);
        bbb = hhh(bbb, ccc, ddd, aaa, x[12], 5);

        ddd = ddd.wrapping_add(cc).wrapping_add(self.status[1]);
        self.status[1] = self.status[2].wrapping_add(dd).wrapping_add(aaa);
        self.status[2] = self.status[3].wrapping_add(aa).wrapping_add(bbb);
        self.status[3] = self.status[0].wrapping_add(bb).wrapping_add(ccc);
        self.status[0] = ddd;
    }
}

impl Default for Ripemd {
    fn default() -> Self {
        Self::new()
    }
}

impl CryptographicHashFunction for Ripemd {
    fn hash_to_bytes(&mut self, message: &[u8]) -> Vec<u8> {
        impl_md_flow!(u32 => self, message, from_le_bytes, to_le_bytes);
        self.status
            .iter()
            .flat_map(|word| word.to_le_bytes().to_vec())
            .collect()
    }
}

impl BlockSize for Ripemd {
    fn byte_block_len(&self) -> usize {
        4 * 16
    }
}

impl DigestSize for Ripemd {
    fn byte_digest_len(&self) -> usize {
        16
    }
}

#[cfg(test)]
mod tests {
    use super::Ripemd;
    use dev_util::impl_test;

    const OFFICIAL: [(&[u8], &str); 7] = [
        // https://archive.org/details/isbn_9783540606406/page/111/mode/1up
        ("".as_bytes(), "9f73aa9b372a9dacfb86a6108852e2d9"),
        ("a".as_bytes(), "486f74f790bc95ef7963cd2382b4bbc9"),
        ("abc".as_bytes(), "3f14bad4c2f9b0ea805e5485d3d6882d"),
        (
            "message digest".as_bytes(),
            "5f5c7ebe1abbb3c7036482942d5f9d49",
        ),
        (
            "abcdefghijklmnopqrstuvwxyz".as_bytes(),
            "ff6e1547494251a1cca6f005a6eaa2b4",
        ),
        (
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".as_bytes(),
            "ff418a5aed3763d8f2ddf88a29e62486",
        ),
        (
            "12345678901234567890123456789012345678901234567890123456789012345678901234567890"
                .as_bytes(),
            "dfd6b45f60fe79bbbde87c6bfc6580a5",
        ),
    ];
    impl_test!(Ripemd, official, OFFICIAL, Ripemd::new());
}
