#![no_std]

extern crate alloc;

use alloc::vec::Vec;

struct ChaChaState<const R: usize>([u32; 16]);

impl<const R: usize> ChaChaState<R> {
    fn initialize(key: &[u8; 32], counter: u32, nonce: &[u8; 12]) -> Self {
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
        Self([
            0x61707865,
            0x3320646e,
            0x79622d32,
            0x6b206574,
            key_word[0],
            key_word[1],
            key_word[2],
            key_word[3],
            key_word[4],
            key_word[5],
            key_word[6],
            key_word[7],
            counter,
            nonce_word[0],
            nonce_word[1],
            nonce_word[2],
        ])
    }
    fn quarter(&mut self, x: usize, y: usize, z: usize, w: usize) {
        let [mut a, mut b, mut c, mut d] = [self.0[x], self.0[y], self.0[z], self.0[w]];

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

        [self.0[x], self.0[y], self.0[z], self.0[w]] = [a, b, c, d];
    }
    fn round(&mut self) {
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
    fn block(&mut self) -> [u32; 16] {
        let mut initial_state = [0; 16];
        initial_state.copy_from_slice(&self.0);
        for _ in 0..R / 2 {
            self.round();
        }
        for i in 0..16 {
            self.0[i] = self.0[i].wrapping_add(initial_state[i]);
        }

        self.0
    }
}

pub struct ChaCha<const R: usize> {
    counter: u32,
    key: [u8; 32],
    nonce: [u8; 12],
    buffer: [u8; 64],
    buffer_length: usize,
    encrypted_message: Vec<u8>,
}

impl<const R: usize> ChaCha<R> {
    const PLAINTEXT_CHUNK_SIZE: usize = 64;
    pub const fn new(key: [u8; 32], nonce: [u8; 12]) -> Self {
        Self {
            counter: 0,
            key,
            nonce,
            buffer: [0; 64],
            buffer_length: 0,
            encrypted_message: Vec::new(),
        }
    }
    fn encrypt_chunk(&mut self) {
        self.counter += 1;
        let key_stream = ChaChaState::<R>::initialize(&self.key, self.counter, &self.nonce).block();

        for i in 0..16 {
            let block = u32::from_le_bytes([
                self.buffer[i * 4],
                self.buffer[i * 4 + 1],
                self.buffer[i * 4 + 2],
                self.buffer[i * 4 + 3],
            ]);
            self.encrypted_message
                .append(&mut (block ^ key_stream[i]).to_le_bytes().to_vec());
        }
    }
    pub fn update(&mut self, plaintext: &[u8]) {
        let plaintext_length = plaintext.len();

        if self.buffer_length + plaintext_length < 64 {
            self.buffer[self.buffer_length..].copy_from_slice(plaintext);
            self.buffer_length += plaintext.len();
            return;
        }

        // encrypt buffer + plaintext
        self.buffer[self.buffer_length..].copy_from_slice(&plaintext[..64 - self.buffer_length]);
        self.encrypt_chunk();
        // encrypt plaintext
        let used_plaintext_head_length = Self::PLAINTEXT_CHUNK_SIZE - self.buffer_length;
        for plaintext_chunk in
            plaintext[used_plaintext_head_length..].chunks_exact(Self::PLAINTEXT_CHUNK_SIZE)
        {
            self.buffer.copy_from_slice(plaintext_chunk);
            self.encrypt_chunk();
        }

        // set unused plaintext to buffer
        let unused_plaintext_length =
            (plaintext_length - used_plaintext_head_length) % Self::PLAINTEXT_CHUNK_SIZE;
        let unused_plaintext = &plaintext[plaintext_length - unused_plaintext_length..];
        self.buffer[..unused_plaintext_length].copy_from_slice(unused_plaintext);
        self.buffer[unused_plaintext_length..].fill(0);
        self.buffer_length = unused_plaintext_length;
    }
    pub fn finalize(&mut self) -> &[u8] {
        if self.buffer_length != 0 {
            self.encrypt_chunk();
        }
        let output_length = self.encrypted_message.len() - (64 - self.buffer_length);
        &self.encrypted_message[0..output_length]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quarter_round() {
        #[rustfmt::skip]
        let mut state = ChaChaState::<1>([
            0x11111111, 0x01020304, 0x9b8d6f43, 0x01234567,
            0, 0, 0, 0,
            0, 0, 0, 0,
            0, 0, 0, 0,
        ]);
        state.quarter(0, 1, 2, 3);
        assert_eq!(
            [state.0[0], state.0[1], state.0[2], state.0[3]],
            [0xea2a92f4, 0xcb1cf8ce, 0x4581472e, 0x5881c4bb]
        );
    }
    #[test]
    fn quarter_round_with_state() {
        #[rustfmt::skip]
        let mut state = ChaChaState::<1>([
            0x8795_31e0, 0xc5ec_f37d,  0x5164_61b1, 0xc9a6_2f8a,
            0x44c2_0ef3, 0x3390_af7f,  0xd9fc_690b, 0x2a5f_714c,
            0x5337_2767, 0xb00a_5631,  0x974c_541a, 0x359e_9963,
            0x5c97_1061, 0x3d63_1689,  0x2098_d9d6, 0x91db_d320,
        ]);
        state.quarter(2, 7, 8, 13);
        #[rustfmt::skip]
        assert_eq!(
            state.0,
            [
                0x8795_31e0, 0xc5ec_f37d,  0xbdb8_86dc, 0xc9a6_2f8a,
                0x44c2_0ef3, 0x3390_af7f,  0xd9fc_690b, 0xcfac_afd2,
                0xe46b_ea80, 0xb00a_5631,  0x974c_541a, 0x359e_9963,
                0x5c97_1061, 0xccc0_7c79,  0x2098_d9d6, 0x91db_d320,
            ]
        );
    }
    #[test]
    fn block() {
        #[rustfmt::skip]
        let key = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
            0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
            0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
        ];
        #[rustfmt::skip]
        let nonce = [
            0x00, 0x00, 0x00, 0x09,
            0x00, 0x00, 0x00, 0x4a,
            0x00, 0x00, 0x00, 0x00,
        ];

        let mut state = ChaChaState::<20>::initialize(&key, 1, &nonce);
        #[rustfmt::skip]
        assert_eq!(
            state.0,
            [
                0x6170_7865, 0x3320_646e, 0x7962_2d32, 0x6b20_6574,
                0x0302_0100, 0x0706_0504, 0x0b0a_0908, 0x0f0e_0d0c,
                0x1312_1110, 0x1716_1514, 0x1b1a_1918, 0x1f1e_1d1c,
                0x0000_0001, 0x0900_0000, 0x4a00_0000, 0x0000_0000,
            ]
        );
        state.block();
        #[rustfmt::skip]
        assert_eq!(
            state.0,
            [
                0xe4e7_f110, 0x1559_3bd1, 0x1fdd_0f50, 0xc471_20a3,
                0xc7f4_d1c7, 0x0368_c033, 0x9aaa_2204, 0x4e6c_d4c3,
                0x4664_82d2, 0x09aa_9f07, 0x05d7_c214, 0xa202_8bd9,
                0xd19c_12b5, 0xb94e_16de, 0xe883_d0cb, 0x4e3c_50a2,
            ]
        );
    }
    #[test]
    fn encrypt() {
        #[rustfmt::skip]
        let key = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
            0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
            0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,            
        ];
        #[rustfmt::skip]
        let nonce = [
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x4a,
            0x00, 0x00, 0x00, 0x00,
        ];

        #[rustfmt::skip]
        let expected = [
            0x6e, 0x2e, 0x35, 0x9a, 0x25, 0x68, 0xf9, 0x80, 0x41, 0xba, 0x07, 0x28, 0xdd, 0x0d, 0x69, 0x81,
            0xe9, 0x7e, 0x7a, 0xec, 0x1d, 0x43, 0x60, 0xc2, 0x0a, 0x27, 0xaf, 0xcc, 0xfd, 0x9f, 0xae, 0x0b,
            0xf9, 0x1b, 0x65, 0xc5, 0x52, 0x47, 0x33, 0xab, 0x8f, 0x59, 0x3d, 0xab, 0xcd, 0x62, 0xb3, 0x57,
            0x16, 0x39, 0xd6, 0x24, 0xe6, 0x51, 0x52, 0xab, 0x8f, 0x53, 0x0c, 0x35, 0x9f, 0x08, 0x61, 0xd8,
            0x07, 0xca, 0x0d, 0xbf, 0x50, 0x0d, 0x6a, 0x61, 0x56, 0xa3, 0x8e, 0x08, 0x8a, 0x22, 0xb6, 0x5e,
            0x52, 0xbc, 0x51, 0x4d, 0x16, 0xcc, 0xf8, 0x06, 0x81, 0x8c, 0xe9, 0x1a, 0xb7, 0x79, 0x37, 0x36,
            0x5a, 0xf9, 0x0b, 0xbf, 0x74, 0xa3, 0x5b, 0xe6, 0xb4, 0x0b, 0x8e, 0xed, 0xf2, 0x78, 0x5e, 0x42,
            0x87, 0x4d
        ];
        let mut chacha = ChaCha::<20>::new(key, nonce);
        let plaintext = b"Ladies and Gentlemen of the class of '99: If I could offer you only one tip for the future, sunscreen would be it.";
        chacha.update(plaintext);
        let result = chacha.finalize();

        assert_eq!(expected, result);
    }
}
