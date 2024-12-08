#![no_std]

mod block;
mod buffer;
mod word;

pub use buffer::Buffer;

#[macro_export]
macro_rules! u32words_from_be_bytes {
    ($block:ident, $bytes:ident) => {
        for i in 0..$block.len() {
            $block[i] = u32::from_be_bytes([
                $bytes[i * 4],
                $bytes[i * 4 + 1],
                $bytes[i * 4 + 2],
                $bytes[i * 4 + 3],
            ])
        }
    };
}

#[macro_export]
macro_rules! u32words_from_le_bytes {
    ($block:ident, $bytes:ident) => {
        for i in 0..$block.len() {
            $block[i] = u32::from_le_bytes([
                $bytes[i * 4],
                $bytes[i * 4 + 1],
                $bytes[i * 4 + 2],
                $bytes[i * 4 + 3],
            ])
        }
    };
}

#[macro_export]
macro_rules! u64words_from_be_bytes {
    ($block:ident, $bytes:ident) => {
        for i in 0..$block.len() {
            $block[i] = u64::from_be_bytes([
                $bytes[i * 8],
                $bytes[i * 8 + 1],
                $bytes[i * 8 + 2],
                $bytes[i * 8 + 3],
                $bytes[i * 8 + 4],
                $bytes[i * 8 + 5],
                $bytes[i * 8 + 6],
                $bytes[i * 8 + 7],
            ])
        }
    };
}

#[macro_export]
macro_rules! u64words_from_le_bytes {
    ($block:ident, $bytes:ident) => {
        for i in 0..$block.len() {
            $block[i] = u64::from_le_bytes([
                $bytes[i * 8],
                $bytes[i * 8 + 1],
                $bytes[i * 8 + 2],
                $bytes[i * 8 + 3],
                $bytes[i * 8 + 4],
                $bytes[i * 8 + 5],
                $bytes[i * 8 + 6],
                $bytes[i * 8 + 7],
            ])
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u32words_from_be_bytes() {
        let mut words = [0; 4];
        #[rustfmt::skip]
        let bytes = [
            0x01, 0x23, 0x45, 0x67,
            0x89, 0xAB, 0xCD, 0xEF,
            0x01, 0x23, 0x45, 0x67,
            0x89, 0xAB, 0xCD, 0xEF,
        ];
        u32words_from_be_bytes!(words, bytes);
        assert_eq!(words[0], 0x0123_4567);
        assert_eq!(words[1], 0x89AB_CDEF);
        assert_eq!(words[2], 0x0123_4567);
        assert_eq!(words[3], 0x89AB_CDEF);
    }
    #[test]
    fn u32words_from_le_bytes() {
        let mut words = [0; 4];
        #[rustfmt::skip]
        let bytes = [
            0x01, 0x23, 0x45, 0x67,
            0x89, 0xAB, 0xCD, 0xEF,
            0x01, 0x23, 0x45, 0x67,
            0x89, 0xAB, 0xCD, 0xEF,
        ];
        u32words_from_le_bytes!(words, bytes);
        assert_eq!(words[0], 0x6745_2301);
        assert_eq!(words[1], 0xEFCD_AB89);
        assert_eq!(words[2], 0x6745_2301);
        assert_eq!(words[3], 0xEFCD_AB89);
    }
    #[test]
    fn u64words_from_be_bytes() {
        let mut words = [0; 2];
        #[rustfmt::skip]
        let bytes = [
            0x01, 0x23, 0x45, 0x67,
            0x89, 0xAB, 0xCD, 0xEF,
            0x01, 0x23, 0x45, 0x67,
            0x89, 0xAB, 0xCD, 0xEF,
        ];
        u64words_from_be_bytes!(words, bytes);
        assert_eq!(words[0], 0x0123_4567_89AB_CDEF);
        assert_eq!(words[1], 0x0123_4567_89AB_CDEF);
    }
    #[test]
    fn u64words_from_le_bytes() {
        let mut words = [0; 2];
        #[rustfmt::skip]
        let bytes = [
            0x01, 0x23, 0x45, 0x67,
            0x89, 0xAB, 0xCD, 0xEF,
            0x01, 0x23, 0x45, 0x67,
            0x89, 0xAB, 0xCD, 0xEF,
        ];
        u64words_from_le_bytes!(words, bytes);
        assert_eq!(words[0], 0xEFCD_AB89_6745_2301);
        assert_eq!(words[1], 0xEFCD_AB89_6745_2301);
    }
}
