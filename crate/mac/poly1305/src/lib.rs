#![no_std]

pub struct Poly1305 {
    s: u128,
    r: [u64; 2],
    a: [u64; 2],
}

impl Poly1305 {
    const BLOCK_SIZE: usize = 16;
    const CLAMP_R_BY: [u64; 2] = [0x0FFF_FFFC_0FFF_FFFC, 0x0FFF_FFFC_0FFF_FFFF];
    // 2^130 - 5 in little endian
    const P: [u64; 3] = [
        0xFFFF_FFFF_FFFF_FFFB,
        0xFFFF_FFFF_FFFF_FFFF,
        0x0000_0000_0000_0003,
    ];

    pub const fn new(key: [u8; 32]) -> Self {
        Self {
            #[rustfmt::skip]
            s: u128::from_le_bytes([
                key[16], key[17], key[18], key[19],
                key[20], key[21], key[22], key[23],
                key[24], key[25], key[26], key[27],
                key[28], key[29], key[30], key[31],
            ]),
            r: [
                u64::from_le_bytes([
                    key[0], key[1], key[2], key[3], key[4], key[5], key[6], key[7],
                ]) & Self::CLAMP_R_BY[0],
                u64::from_le_bytes([
                    key[8], key[9], key[10], key[11], key[12], key[13], key[14], key[15],
                ]) & Self::CLAMP_R_BY[1],
            ],
            a: [0; 2],
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
                    bytes[i * 16 + 15],
                ]);
                let a = ((self.a[0] as u128).wrapping_shl(64) & self.a[1] as u128).wrapping_add(n);
                self.a = [
                    (a.wrapping_shr(64) as u64) & u64::MAX,
                    (a as u64) & u64::MAX,
                ];
                let a0r0 = (self.r[0] as u128).wrapping_mul(rhs);
                self.a = self
                    .r
                    .wrapping_mul(self.a)
                    .wrapping_rem(0x3fff_ffff_ffff_ffff_ffff_ffff_ffff_ffffb)
            })
    }
    pub fn mac(key: [u8; 32], message: &[u8]) {
        #[rustfmt::skip]

        let mut acc = 0;

        for i in 1..=message.len() % 16 {
            let n = u128::from_be_bytes([
                message[(i - 1) * 16],
                message[(i - 1) * 16],
                message[(i - 1) * 16],
                message[(i - 1) * 16],
                message[(i - 1) * 16],
                message[(i - 1) * 16],
                message[(i - 1) * 16],
                message[(i - 1) * 16],
                message[(i - 1) * 16],
                message[(i - 1) * 16],
                message[(i - 1) * 16],
                message[(i - 1) * 16],
                message[(i - 1) * 16],
                message[(i - 1) * 16],
                message[(i - 1) * 16],
                message[i * 16],
            ]);
        }
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
