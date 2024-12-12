mod lake256;

pub use lake256::Lake256;

use core::cmp::Ordering;

use util::{impl_md_flow, uint_from_bytes};

use crate::consts::{C32, C64, IV256, IV512};

pub struct Lake<T> {
    s: [T; 4], // salt
    l: [T; 16],
    h: [T; 8],
    t: [T; 2], // counter: 処理したビット数(と次に処理をするブロックのビット数?)
    r: usize,  // round
    processed_bytes: usize,
}

const fn f32(a: u32, b: u32, c: u32, d: u32) -> u32 {
    a.wrapping_add(b | C32[0])
        .wrapping_add(c.wrapping_add(a & C32[1]).rotate_right(7))
        .wrapping_add(b.wrapping_add(c ^ d).rotate_right(13))
}

const fn g32(a: u32, b: u32, c: u32, d: u32) -> u32 {
    a.wrapping_add(b).rotate_right(1) ^ c.wrapping_add(d)
}

impl Lake<u32> {
    const fn new(h: [u32; 8], s: [u32; 4], r: usize) -> Self {
        Self {
            s,
            l: [0; 16],
            h,
            t: [0; 2],
            r,
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
        self.t[0] += 1;
        // if self.processed_bytes > 512 {
        //     self.t[0] += 512;
        //     self.processed_bytes -= 512;
        // } else {
        //     self.t[0] += self.processed_bytes as u32;
        //     self.processed_bytes = 0;
        // }

        // 1.
        self.salt_state();
        // 2.
        for _ in 0..self.r {
            self.process_message(block);
        }
        // 3.
        self.feed_forward();
    }
    fn lake(&mut self, message: &[u8]) {
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
                32,
                0,
                0,
                0,
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
                    byte_block[112..].copy_from_slice(&(8 * l as u64).to_le_bytes());
                    byte_block[120..].copy_from_slice(&(8 * 32 as u64).to_le_bytes());
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
                    byte_block[48..].copy_from_slice(&(8 * l as u64).to_le_bytes());
                    byte_block[56..].copy_from_slice(&(8 * 32 as u64).to_le_bytes());
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
