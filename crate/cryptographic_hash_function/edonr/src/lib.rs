#![no_std]

extern crate alloc;

mod consts;
mod edonr224;
mod edonr256;
mod edonr384;
mod edonr512;

use core::cmp::Ordering;

use cryptographic_hash_function::BlockSize;
use merkle_damgard::impl_md_flow;

use consts::{P224, P256, P384, P512};

pub use edonr224::EdonR224;
pub use edonr256::EdonR256;
pub use edonr384::EdonR384;
pub use edonr512::EdonR512;

#[inline(always)]
pub(crate) const fn q256(x: &[u32; 8], y: &[u32; 8]) -> [u32; 8] {
    let mut z = [0u32; 8];
    let mut t = [0u32; 16];
    // First Latin Square
    t[0] = x[0]
        .wrapping_add(x[1])
        .wrapping_add(x[2])
        .wrapping_add(x[4])
        .wrapping_add(x[7])
        .wrapping_add(0xAAAA_AAAA);
    t[1] = x[0]
        .wrapping_add(x[1])
        .wrapping_add(x[3])
        .wrapping_add(x[4])
        .wrapping_add(x[7])
        .rotate_left(5);
    t[2] = x[0]
        .wrapping_add(x[1])
        .wrapping_add(x[4])
        .wrapping_add(x[6])
        .wrapping_add(x[7])
        .rotate_left(11);
    t[3] = x[2]
        .wrapping_add(x[3])
        .wrapping_add(x[5])
        .wrapping_add(x[6])
        .wrapping_add(x[7])
        .rotate_left(13);
    t[4] = x[1]
        .wrapping_add(x[2])
        .wrapping_add(x[3])
        .wrapping_add(x[5])
        .wrapping_add(x[6])
        .rotate_left(17);
    t[5] = x[0]
        .wrapping_add(x[2])
        .wrapping_add(x[3])
        .wrapping_add(x[4])
        .wrapping_add(x[5])
        .rotate_left(19);
    t[6] = x[0]
        .wrapping_add(x[1])
        .wrapping_add(x[5])
        .wrapping_add(x[6])
        .wrapping_add(x[7])
        .rotate_left(29);
    t[7] = x[2]
        .wrapping_add(x[3])
        .wrapping_add(x[4])
        .wrapping_add(x[5])
        .wrapping_add(x[6])
        .rotate_left(31);
    t[8] = t[3] ^ t[5] ^ t[6];
    t[9] = t[2] ^ t[5] ^ t[6];
    t[10] = t[2] ^ t[3] ^ t[5];
    t[11] = t[0] ^ t[1] ^ t[4];
    t[12] = t[0] ^ t[4] ^ t[7];
    t[13] = t[1] ^ t[6] ^ t[7];
    t[14] = t[2] ^ t[3] ^ t[4];
    t[15] = t[0] ^ t[1] ^ t[7];
    // Second Orthogonal Latin Square
    t[0] = y[0]
        .wrapping_add(y[1])
        .wrapping_add(y[2])
        .wrapping_add(y[5])
        .wrapping_add(y[7])
        .wrapping_add(0x5555_5555);
    t[1] = y[0]
        .wrapping_add(y[1])
        .wrapping_add(y[3])
        .wrapping_add(y[4])
        .wrapping_add(y[6])
        .rotate_left(3);
    t[2] = y[0]
        .wrapping_add(y[1])
        .wrapping_add(y[2])
        .wrapping_add(y[3])
        .wrapping_add(y[5])
        .rotate_left(7);
    t[3] = y[2]
        .wrapping_add(y[3])
        .wrapping_add(y[4])
        .wrapping_add(y[6])
        .wrapping_add(y[7])
        .rotate_left(11);
    t[4] = y[0]
        .wrapping_add(y[1])
        .wrapping_add(y[3])
        .wrapping_add(y[4])
        .wrapping_add(y[5])
        .rotate_left(17);
    t[5] = y[2]
        .wrapping_add(y[4])
        .wrapping_add(y[5])
        .wrapping_add(y[6])
        .wrapping_add(y[7])
        .rotate_left(19);
    t[6] = y[1]
        .wrapping_add(y[2])
        .wrapping_add(y[5])
        .wrapping_add(y[6])
        .wrapping_add(y[7])
        .rotate_left(23);
    t[7] = y[0]
        .wrapping_add(y[3])
        .wrapping_add(y[4])
        .wrapping_add(y[6])
        .wrapping_add(y[7])
        .rotate_left(29);
    z[5] = t[8].wrapping_add(t[3] ^ t[4] ^ t[6]);
    z[6] = t[9].wrapping_add(t[2] ^ t[5] ^ t[7]);
    z[7] = t[10].wrapping_add(t[4] ^ t[6] ^ t[7]);
    z[0] = t[11].wrapping_add(t[0] ^ t[1] ^ t[5]);
    z[1] = t[12].wrapping_add(t[2] ^ t[6] ^ t[7]);
    z[2] = t[13].wrapping_add(t[0] ^ t[1] ^ t[3]);
    z[3] = t[14].wrapping_add(t[0] ^ t[3] ^ t[4]);
    z[4] = t[15].wrapping_add(t[1] ^ t[2] ^ t[5]);
    z
}
#[inline(always)]
pub(crate) const fn q512(x: &[u64; 8], y: &[u64; 8]) -> [u64; 8] {
    let mut z = [0u64; 8];
    let mut t = [0u64; 16];
    // First Latin Square
    t[0] = x[0]
        .wrapping_add(x[1])
        .wrapping_add(x[2])
        .wrapping_add(x[4])
        .wrapping_add(x[7])
        .wrapping_add(0xAAAA_AAAA_AAAA_AAAA);
    t[1] = x[0]
        .wrapping_add(x[1])
        .wrapping_add(x[3])
        .wrapping_add(x[4])
        .wrapping_add(x[7])
        .rotate_left(5);
    t[2] = x[0]
        .wrapping_add(x[1])
        .wrapping_add(x[4])
        .wrapping_add(x[6])
        .wrapping_add(x[7])
        .rotate_left(19);
    t[3] = x[2]
        .wrapping_add(x[3])
        .wrapping_add(x[5])
        .wrapping_add(x[6])
        .wrapping_add(x[7])
        .rotate_left(29);
    t[4] = x[1]
        .wrapping_add(x[2])
        .wrapping_add(x[3])
        .wrapping_add(x[5])
        .wrapping_add(x[6])
        .rotate_left(31);
    t[5] = x[0]
        .wrapping_add(x[2])
        .wrapping_add(x[3])
        .wrapping_add(x[4])
        .wrapping_add(x[5])
        .rotate_left(41);
    t[6] = x[0]
        .wrapping_add(x[1])
        .wrapping_add(x[5])
        .wrapping_add(x[6])
        .wrapping_add(x[7])
        .rotate_left(57);
    t[7] = x[2]
        .wrapping_add(x[3])
        .wrapping_add(x[4])
        .wrapping_add(x[5])
        .wrapping_add(x[6])
        .rotate_left(61);
    t[8] = t[3] ^ t[5] ^ t[6];
    t[9] = t[2] ^ t[5] ^ t[6];
    t[10] = t[2] ^ t[3] ^ t[5];
    t[11] = t[0] ^ t[1] ^ t[4];
    t[12] = t[0] ^ t[4] ^ t[7];
    t[13] = t[1] ^ t[6] ^ t[7];
    t[14] = t[2] ^ t[3] ^ t[4];
    t[15] = t[0] ^ t[1] ^ t[7];
    // Second Orthogonal Latin Square
    t[0] = y[0]
        .wrapping_add(y[1])
        .wrapping_add(y[2])
        .wrapping_add(y[5])
        .wrapping_add(y[7])
        .wrapping_add(0x5555_5555_5555_5555);
    t[1] = y[0]
        .wrapping_add(y[1])
        .wrapping_add(y[3])
        .wrapping_add(y[4])
        .wrapping_add(y[6])
        .rotate_left(3);
    t[2] = y[0]
        .wrapping_add(y[1])
        .wrapping_add(y[2])
        .wrapping_add(y[3])
        .wrapping_add(y[5])
        .rotate_left(17);
    t[3] = y[2]
        .wrapping_add(y[3])
        .wrapping_add(y[4])
        .wrapping_add(y[6])
        .wrapping_add(y[7])
        .rotate_left(23);
    t[4] = y[0]
        .wrapping_add(y[1])
        .wrapping_add(y[3])
        .wrapping_add(y[4])
        .wrapping_add(y[5])
        .rotate_left(31);
    t[5] = y[2]
        .wrapping_add(y[4])
        .wrapping_add(y[5])
        .wrapping_add(y[6])
        .wrapping_add(y[7])
        .rotate_left(37);
    t[6] = y[1]
        .wrapping_add(y[2])
        .wrapping_add(y[5])
        .wrapping_add(y[6])
        .wrapping_add(y[7])
        .rotate_left(45);
    t[7] = y[0]
        .wrapping_add(y[3])
        .wrapping_add(y[4])
        .wrapping_add(y[6])
        .wrapping_add(y[7])
        .rotate_left(59);
    z[5] = t[8].wrapping_add(t[3] ^ t[4] ^ t[6]);
    z[6] = t[9].wrapping_add(t[2] ^ t[5] ^ t[7]);
    z[7] = t[10].wrapping_add(t[4] ^ t[6] ^ t[7]);
    z[0] = t[11].wrapping_add(t[0] ^ t[1] ^ t[5]);
    z[1] = t[12].wrapping_add(t[2] ^ t[6] ^ t[7]);
    z[2] = t[13].wrapping_add(t[0] ^ t[1] ^ t[3]);
    z[3] = t[14].wrapping_add(t[0] ^ t[3] ^ t[4]);
    z[4] = t[15].wrapping_add(t[1] ^ t[2] ^ t[5]);
    z
}

#[derive(Clone, Copy)]
struct EdonR<T> {
    state: [T; 16],
}

impl EdonR<u32> {
    const fn new(iv: [u32; 16]) -> Self {
        Self { state: iv }
    }
    #[inline(always)]
    fn compress(&mut self, message: &[u32; 16]) {
        let mut state_8 = [
            self.state[0],
            self.state[1],
            self.state[2],
            self.state[3],
            self.state[4],
            self.state[5],
            self.state[6],
            self.state[7],
        ];
        let mut state_16 = [
            self.state[8],
            self.state[9],
            self.state[10],
            self.state[11],
            self.state[12],
            self.state[13],
            self.state[14],
            self.state[15],
        ];
        // First row of quasigroup e-transformations
        let mut state_24 = q256(
            &[
                message[15],
                message[14],
                message[13],
                message[12],
                message[11],
                message[10],
                message[9],
                message[8],
            ],
            &[
                message[0], message[1], message[2], message[3], message[4], message[5], message[6],
                message[7],
            ],
        );
        let mut state_32 = q256(
            &state_24,
            &[
                message[8],
                message[9],
                message[10],
                message[11],
                message[12],
                message[13],
                message[14],
                message[15],
            ],
        );
        // Second row of quasigroup e-transformations
        state_24 = q256(&state_16, &state_24);
        state_32 = q256(&state_24, &state_32);
        // Third row of quasigroup e-transformations
        state_24 = q256(&state_24, &state_8);
        state_32 = q256(&state_32, &state_24);
        // Fourth row of quasigroup e-transformations
        state_8 = q256(
            &[
                message[7], message[6], message[5], message[4], message[3], message[2], message[1],
                message[0],
            ],
            &state_24,
        );
        state_16 = q256(&state_8, &state_32);
        self.state[0..8].copy_from_slice(&state_8);
        self.state[8..16].copy_from_slice(&state_16);
    }
    fn edonr(&mut self, message: &[u8]) {
        impl_md_flow!(u32=> self, message, from_le_bytes, to_le_bytes);
    }
}

impl BlockSize for EdonR<u32> {
    fn byte_block_len(&self) -> usize {
        4 * 16
    }
}

impl EdonR<u64> {
    const fn new(iv: [u64; 16]) -> Self {
        Self { state: iv }
    }
    #[inline(always)]
    fn compress(&mut self, message: &[u64; 16]) {
        let mut state_8 = [
            self.state[0],
            self.state[1],
            self.state[2],
            self.state[3],
            self.state[4],
            self.state[5],
            self.state[6],
            self.state[7],
        ];
        let mut state_16 = [
            self.state[8],
            self.state[9],
            self.state[10],
            self.state[11],
            self.state[12],
            self.state[13],
            self.state[14],
            self.state[15],
        ];
        // First row of quasigroup e-transformations
        let mut state_24 = q512(
            &[
                message[15],
                message[14],
                message[13],
                message[12],
                message[11],
                message[10],
                message[9],
                message[8],
            ],
            &[
                message[0], message[1], message[2], message[3], message[4], message[5], message[6],
                message[7],
            ],
        );
        let mut state_32 = q512(
            &state_24,
            &[
                message[8],
                message[9],
                message[10],
                message[11],
                message[12],
                message[13],
                message[14],
                message[15],
            ],
        );
        // Second row of quasigroup e-transformations
        state_24 = q512(&state_16, &state_24);
        state_32 = q512(&state_24, &state_32);
        // Third row of quasigroup e-transformations
        state_24 = q512(&state_24, &state_8);
        state_32 = q512(&state_32, &state_24);
        // Fourth row of quasigroup e-transformations
        state_8 = q512(
            &[
                message[7], message[6], message[5], message[4], message[3], message[2], message[1],
                message[0],
            ],
            &state_24,
        );
        state_16 = q512(&state_8, &state_32);
        self.state[0..8].copy_from_slice(&state_8);
        self.state[8..16].copy_from_slice(&state_16);
    }
    // EDON-R{384, 512}は、パディング末尾に付与するビット長が64bit分なので`impl_md_flow`マクロを利用していない。
    fn edonr(&mut self, message: &[u8]) {
        let l = message.len();
        let mut block = [0u64; 16];
        if l >= 128 {
            message.chunks_exact(128).for_each(|bytes| {
                (0..16).for_each(|i| {
                    block[i] = u64::from_le_bytes([
                        bytes[i * 8],
                        bytes[i * 8 + 1],
                        bytes[i * 8 + 2],
                        bytes[i * 8 + 3],
                        bytes[i * 8 + 4],
                        bytes[i * 8 + 5],
                        bytes[i * 8 + 6],
                        bytes[i * 8 + 7],
                    ]);
                });
                self.compress(&block);
            });
        } else if l == 0 {
            self.compress(&[
                u64::from_le_bytes([0x80, 0, 0, 0, 0, 0, 0, 0]),
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
                0,
            ])
        }
        if l != 0 {
            let offset = (l / 128) * 128;
            let remainder = l % 128;
            match (l % 128).cmp(&119) {
                Ordering::Greater => {
                    // two blocks
                    let mut byte_block = [0u8; 256];
                    byte_block[..remainder].copy_from_slice(&message[offset..]);
                    byte_block[remainder] = 0x80;
                    byte_block[248..].copy_from_slice(&(8 * l as u64).to_le_bytes());
                    byte_block.chunks_exact(128).for_each(|bytes| {
                        (0..16).for_each(|i| {
                            block[i] = u64::from_le_bytes([
                                bytes[i * 8],
                                bytes[i * 8 + 1],
                                bytes[i * 8 + 2],
                                bytes[i * 8 + 3],
                                bytes[i * 8 + 4],
                                bytes[i * 8 + 5],
                                bytes[i * 8 + 6],
                                bytes[i * 8 + 7],
                            ]);
                        });
                        self.compress(&block);
                    });
                }
                Ordering::Less | Ordering::Equal => {
                    // one block
                    let mut byte_block = [0u8; 128];
                    byte_block[..remainder].copy_from_slice(&message[offset..]);
                    byte_block[remainder] = 0x80;
                    byte_block[120..].copy_from_slice(&(8 * l as u64).to_le_bytes());
                    (0..16).for_each(|i| {
                        block[i] = u64::from_le_bytes([
                            byte_block[i * 8],
                            byte_block[i * 8 + 1],
                            byte_block[i * 8 + 2],
                            byte_block[i * 8 + 3],
                            byte_block[i * 8 + 4],
                            byte_block[i * 8 + 5],
                            byte_block[i * 8 + 6],
                            byte_block[i * 8 + 7],
                        ]);
                    });
                    self.compress(&block);
                }
            }
        }
    }
}

impl BlockSize for EdonR<u64> {
    fn byte_block_len(&self) -> usize {
        8 * 16
    }
}
