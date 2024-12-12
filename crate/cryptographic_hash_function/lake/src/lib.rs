mod lake256;

pub use lake256::Lake256;

use core::cmp::Ordering;

pub struct Lake<T> {
    s: [T; 4], // salt
    l: [T; 16],
    h: [T; 8],
    t: [T; 2],
    processed_bytes: usize,
}

// LAKE-256
#[rustfmt::skip]
pub(crate)const C32: [u32; 16] = [
    0x4528_21E6, 0x38D0_1377, 0xBE54_66CF, 0x34E9_0C6C, 0xC0AC_29B7, 0xC97C_50DD, 0x3F84_D5B5, 0xB547_0917,
    0x9216_D5D9, 0x8979_FB1B, 0xD131_0BA6, 0x98DF_B5AC, 0x2FFD_72DB, 0xD01A_DFB7, 0xB8E1_AFED, 0x6A26_7E96
];
// LAKE-512
#[rustfmt::skip]
pub(crate)const C64: [u64; 16] = [
    0x0769_441A_D54C_789F, 0x3CB6_2BB7_21C2_746E, 0x1BE9_73B3_FF6C_5EDE, 0xD988_3F66_6CD3_7F6B,
    0x2A95_7219_3E06_AA68, 0x8AB8_7CA9_2226_05F2, 0x3B43_E1D7_013C_EAC5, 0xDF65_34E1_E77E_037E,
    0x4623_A40A_B23A_2E02, 0xA43B_A7CD_FC9B_CF82, 0xD6AE_BF43_FB26_6C5E, 0x1393_6309_7AAB_1247,
    0x2A53_B4E0_A95C_AA01, 0x8D17_7071_4B74_9520, 0xB3BC_88DB_689C_A207, 0xC46E_F390_31B3_E5A5
];

// LAKE-256
#[rustfmt::skip]
pub(crate) const IV256: [u32; 8] = [
    0x243F_6A88, 0x85A3_08D3, 0x131_98A2E, 0x0370_7344,
    0xA409_3822, 0x299F_31D0, 0x082_EFA98, 0xEC4E_6C89
];
// LAKE-512
#[rustfmt::skip]
pub(crate) const IV512: [u64; 8] = [
    0x57F5_C7D0_8881_3AFC, 0x1390_8A7C_25E9_45C0, 0xB273_D634_AF46_35AB, 0xB8E6_A0E2_AE02_5B8F,
    0xF92F_3FFE_B779_0C39, 0x428D_3FD1_A930_A4EE, 0xA66C_46E2_B325_5458, 0xF2AC_54FE_DE1E_C2EA
];

const fn f32(a: u32, b: u32, c: u32, d: u32) -> u32 {
    a.wrapping_add(b | C32[0])
        .wrapping_add(c.wrapping_add(a & C32[1]).rotate_right(7))
        .wrapping_add(b.wrapping_add(c ^ d).rotate_right(13))
}

const fn g32(a: u32, b: u32, c: u32, d: u32) -> u32 {
    a.wrapping_add(b).rotate_right(1) ^ c.wrapping_add(d)
}

impl Lake<u32> {
    const R: usize = 8;
    // const D: u6 = 256;

    const fn new(h: [u32; 8], s: [u32; 4]) -> Self {
        Self {
            s,
            l: [0; 16],
            h,
            t: [0; 2],
            processed_bytes: 0,
        }
    }
    fn salt_state(&mut self) {
        self.l = [
            // 1.
            self.h[0],
            self.h[1],
            self.h[2],
            self.h[3],
            self.h[4],
            self.h[5],
            self.h[6],
            self.h[7],
            // 2.
            g32(self.h[0], self.s[0] ^ self.t[0], C32[8], 0),
            // 3.
            g32(self.h[1], self.s[1] ^ self.t[1], C32[9], 0),
            // 4.
            g32(self.h[10 % 8], self.s[2], C32[10], 0),
            g32(self.h[11 % 8], self.s[3], C32[11], 0),
            g32(self.h[12 % 8], self.s[0], C32[12], 0),
            g32(self.h[13 % 8], self.s[1], C32[13], 0),
            g32(self.h[14 % 8], self.s[2], C32[14], 0),
            g32(self.h[15 % 8], self.s[3], C32[15], 0),
        ];
    }
    fn process_message(&mut self, block: &[u32; 16]) {
        // 1.
        let f = self.l;
        // 2.
        for i in 0..16 {
            self.l[i] = f32(self.l[(i + 16 - 1) % 16], self.l[i], block[i], C32[i]);
        }
        // 3.
        for i in 0..16 {
            self.l[i] = g32(
                self.l[(i + 16 - 1) % 16],
                self.l[i],
                f[i],
                self.l[(i + 1) % 16],
            );
        }
    }
    fn feed_forward(&mut self) {
        self.h = [
            // 1.
            f32(self.l[0], self.l[8], self.s[0] ^ self.t[0], self.h[0]),
            f32(self.l[1], self.l[9], self.s[1] ^ self.t[1], self.h[1]),
            // 2.
            f32(self.l[2], self.l[2 + 8], self.s[2 % 4], self.h[2]),
            f32(self.l[3], self.l[3 + 8], self.s[3 % 4], self.h[3]),
            f32(self.l[4], self.l[4 + 8], self.s[4 % 4], self.h[4]),
            f32(self.l[5], self.l[5 + 8], self.s[5 % 4], self.h[5]),
            f32(self.l[6], self.l[6 + 8], self.s[6 % 4], self.h[6]),
            f32(self.l[7], self.l[7 + 8], self.s[7 % 4], self.h[7]),
        ];
    }
    fn compress(&mut self, block: &[u32; 16]) {
        // 1.
        self.salt_state();
        // 2.
        for _ in 0..Self::R {
            self.process_message(block);
        }
        // 3.
        self.feed_forward();

        // self.t[0] += 1;
    }
    fn lake(&mut self, message: &[u8]) {
        self.compress(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 256]);
        let l = message.len();
        let mut block = [0u32; 16];
        if l >= 64 {
            message.chunks_exact(64).for_each(|bytes| {
                (0..16).for_each(|i| {
                    block[i] = u32::from_le_bytes([
                        bytes[i * 4],
                        bytes[i * 4 + 1],
                        bytes[i * 4 + 2],
                        bytes[i * 4 + 3],
                    ]);
                });
                self.compress(&block);
            });
        } else if l == 0 {
            self.compress(&[
                u32::from_le_bytes([0x80, 0, 0, 0]),
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                256u32.to_le(),
            ])
        }
        if l != 0 {
            let offset = (l / 64) * 64;
            let remainder = l % 64;
            match (l % 64).cmp(&47) {
                Ordering::Greater => {
                    // two blocks
                    let mut byte_block = [0u8; 128];
                    byte_block[..remainder].copy_from_slice(&message[offset..]);
                    byte_block[remainder] = 0x80;
                    // byte_block[112..].copy_from_slice(&(8 * l as u64).to_le_bytes());
                    byte_block[120..].copy_from_slice(&(8 * l as u64).to_le_bytes());
                    // byte_block[120..].copy_from_slice(&(8 * 32 as u64).to_le_bytes());
                    byte_block.chunks_exact(64).for_each(|bytes| {
                        (0..16).for_each(|i| {
                            block[i] = u32::from_le_bytes([
                                bytes[i * 4],
                                bytes[i * 4 + 1],
                                bytes[i * 4 + 2],
                                bytes[i * 4 + 3],
                            ]);
                        });
                        self.compress(&block);
                    });
                }
                Ordering::Less | Ordering::Equal => {
                    // one block
                    let mut byte_block = [0u8; 64];
                    byte_block[..remainder].copy_from_slice(&message[offset..]);
                    byte_block[remainder] = 0x80;
                    // byte_block[48..].copy_from_slice(&(8 * l as u64).to_le_bytes());
                    // byte_block[56..].copy_from_slice(&(8 * 32 as u64).to_le_bytes());
                    // byte_block[56..].copy_from_slice(&((8 * l as u64) + 256).to_le_bytes());
                    (0..16).for_each(|i| {
                        block[i] = u32::from_le_bytes([
                            byte_block[i * 4],
                            byte_block[i * 4 + 1],
                            byte_block[i * 4 + 2],
                            byte_block[i * 4 + 3],
                        ]);
                    });
                    self.compress(&block);
                }
            }
        }
    }
}
