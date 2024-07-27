#![no_std]

pub fn hello() -> u32 {
    1 + 1
}

struct ChaChaBlock<const R: usize> {
    state: [u32; 16],
}

impl<const R: usize> ChaChaBlock<R> {
    pub fn new(key: [u8; 32], nonce: [u8; 12]) -> Self {
        let mut key_word = [0; 8];
        for i in 0..8 {
            key_word[i] =
                u32::from_le_bytes([key[i * 4], key[i * 4 + 1], key[i * 4 + 2], key[i * 4 + 3]]);
        }
        let mut nonce_word = [0; 3];
        for i in 0..3 {
            nonce_word[i] = u32::from_le_bytes([
                nonce[i * 4],
                nonce[i * 4 + 1],
                nonce[i * 4 + 2],
                nonce[i * 4 + 3],
            ]);
        }
        Self {
            #[rustfmt::skip]
            state: [
                0x61707865, 0x3320646e, 0x79622d32, 0x6b206574,
                key_word[0], key_word[1], key_word[2], key_word[3],
                key_word[4], key_word[5], key_word[6], key_word[7],
                1, nonce_word[0], nonce_word[1], nonce_word[2]
            ],
        }
    }
    fn quarter(&mut self, x: usize, y: usize, z: usize, w: usize) {
        let [mut a, mut b, mut c, mut d] =
            [self.state[x], self.state[y], self.state[z], self.state[w]];

        a = a.wrapping_add(b);
        d ^= a;
        d = d.rotate_left(16);

        c = c.wrapping_add(d);
        b ^= c;
        b = b.rotate_left(12);
        a = a.wrapping_add(b);
        d ^= a;
        d = d.rotate_left(8);

        c = c.wrapping_add(d);
        b ^= c;
        b = b.rotate_left(7);

        [self.state[x], self.state[y], self.state[z], self.state[w]] = [a, b, c, d];
    }
    fn round(&mut self) {
        for _ in 0..R {
            // column round
            self.quarter(0, 4, 8, 12);
            self.quarter(1, 5, 9, 13);
            self.quarter(2, 6, 10, 14);
            self.quarter(3, 7, 11, 15);
            // diagonal round
            self.quarter(0, 5, 10, 15);
            self.quarter(1, 6, 11, 12);
            self.quarter(2, 7, 8, 13);
            self.quarter(3, 4, 9, 14);
        }
    }
    fn compress(&mut self) {
        let mut initial_state = [0; 16];
        initial_state.copy_from_slice(&self.state);
        for _ in 0..10 {
            self.round();
        }
        for i in 0..16 {
            self.state[i] = self.state[i].wrapping_add(initial_state[i]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quarter() {
        let [a, b, c, d] = [0x11111111, 0x01020304, 0x9b8d6f43, 0x01234567];
        let mut state = [0; 16];
        state[0..4].copy_from_slice(&[a, b, c, d]);
        let mut block = ChaChaBlock::<1> { state };
        block.quarter(0, 1, 2, 3);
        assert_eq!(
            [
                block.state[0],
                block.state[1],
                block.state[2],
                block.state[3]
            ],
            [0xea2a92f4, 0xcb1cf8ce, 0x4581472e, 0x5881c4bb]
        );
    }
    #[test]
    fn round() {
        #[rustfmt::skip]
        let state = [
            0x8795_31e0, 0xc5ec_f37d,  0x5164_61b1, 0xc9a6_2f8a,
            0x44c2_0ef3, 0x3390_af7f,  0xd9fc_690b, 0x2a5f_714c,
            0x5337_2767, 0xb00a_5631,  0x974c_541a, 0x359e_9963,
            0x5c97_1061, 0x3d63_1689,  0x2098_d9d6, 0x91db_d320,
        ];
        let mut block = ChaChaBlock::<1> { state };
        block.quarter(2, 7, 8, 13);
        #[rustfmt::skip]
        assert_eq!(
            block.state,
            [
                0x8795_31e0, 0xc5ec_f37d,  0xbdb8_86dc, 0xc9a6_2f8a,
                0x44c2_0ef3, 0x3390_af7f,  0xd9fc_690b, 0xcfac_afd2,
                0xe46b_ea80, 0xb00a_5631,  0x974c_541a, 0x359e_9963,
                0x5c97_1061, 0xccc0_7c79,  0x2098_d9d6, 0x91db_d320,
            ]
        );
    }
    #[test]
    fn compress() {
        let key = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
            0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b,
            0x1c, 0x1d, 0x1e, 0x1f,
        ];
        let nonce = [
            0x00, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x4a, 0x00, 0x00, 0x00, 0x00,
        ];

        let mut block = ChaChaBlock::<20>::new(key, nonce);
        #[rustfmt::skip]
        assert_eq!(
            block.state,
            [
                0x6170_7865, 0x3320_646e, 0x7962_2d32, 0x6b20_6574,
                0x0302_0100, 0x0706_0504, 0x0b0a_0908, 0x0f0e_0d0c,
                0x1312_1110, 0x1716_1514, 0x1b1a_1918, 0x1f1e_1d1c,
                0x0000_0001, 0x0900_0000, 0x4a00_0000, 0x0000_0000,
            ]
        );
        block.compress();
        #[rustfmt::skip]
        assert_eq!(
            block.state,
            [
                // 0x8377_78ab, 0xe238_d763, 0xa67a_e21e, 0x5950_bb2f,
                // 0xc4f2_d0c7, 0xfc62_bb2f, 0x8fa0_18fc, 0x3f5e_c7b7,
                // 0x3352_71c2, 0xf294_89f3, 0xeabd_a8fc, 0x82e4_6ebd,
                // 0xd19c_12b4, 0xb04e_16de, 0x9e83_d0cb, 0x4e3c_50a2,
                0xe4e7_f110, 0x1559_3bd1, 0x1fdd_0f50, 0xc471_20a3,
                0xc7f4_d1c7, 0x0368_c033, 0x9aaa_2204, 0x4e6c_d4c3,
                0x4664_82d2, 0x09aa_9f07, 0x05d7_c214, 0xa202_8bd9,
                0xd19c_12b5, 0xb94e_16de, 0xe883_d0cb, 0x4e3c_50a2,
            ]
        );
    }
}
