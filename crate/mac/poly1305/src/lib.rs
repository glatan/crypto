#![no_std]

use bignumber::UnsignedBigNumber;

pub struct Poly1305 {
    s: u128,
    r: UnsignedBigNumber,
    a: UnsignedBigNumber,
    p: UnsignedBigNumber,
}

impl Poly1305 {
    const CLAMP_R_BY: u128 = 0x0FFF_FFFC_0FFF_FFFC_0FFF_FFFC_0FFF_FFFF;
    // 2^130 - 5 in little endian
    const P: [u128; 2] = [
        0x0000_0000_0000_0000_FFFF_FFFF_FFFF_FFFB,
        0xFFFF_FFFF_FFFF_FFFF_0000_0000_0000_0003,
    ];

    pub fn new(key: [u8; 32]) -> Self {
        Self {
            #[rustfmt::skip]
            s: u128::from_le_bytes([
                key[16], key[17], key[18], key[19],
                key[20], key[21], key[22], key[23],
                key[24], key[25], key[26], key[27],
                key[28], key[29], key[30], key[31],
            ]),
            r: UnsignedBigNumber::new(&[u128::from_le_bytes([key[0], key[1], key[2], key[3], key[4], key[5], key[6], key[7], key[8], key[9], key[10], key[11], key[12], key[13], key[14], key[15]]) & Self::CLAMP_R_BY]),
            a: UnsignedBigNumber::new(&[0]),
            p: UnsignedBigNumber::new(&Self::P),
        }
    }
    fn update(&mut self, message: &[u8]) {
        message
            .chunks_exact(Self::BLOCK_SIZE)
            .enumerate()
            .for_each(|(i, bytes)| {
                let n = u128::from_le_bytes([
                    bytes[i * 16],
                    bytes[i * 16 + 1],
                    bytes[i * 16 + 2],
                    bytes[i * 16 + 3],
                    bytes[i * 16 + 4],
                    bytes[i * 16 + 5],
                    bytes[i * 16 + 6],
                    bytes[i * 16 + 7],
                    bytes[i * 16 + 8],
                    bytes[i * 16 + 9],
                    bytes[i * 16 + 10],
                    bytes[i * 16 + 11],
                    bytes[i * 16 + 12],
                    bytes[i * 16 + 13],
                    bytes[i * 16 + 14],
                    bytes[i * 16 + 15] & 0x01,
                ]);
                self.a.push(n);
                self.a = (&self.r * &self.a) % &self.p;
            })
    }
    pub fn finalize(&mut self) {
        self.a.push(self.s);
    }
}

