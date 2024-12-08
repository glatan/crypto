use core::marker::PhantomData;

use crate::Buffer;

pub struct Word<T, const N: usize> {
    _endian: PhantomData<WordEndian>,
    _word: PhantomData<T>,
}

// impl<const N:usize> Word<u32, N> {
//     pub fn from_bytes(buffer: Buffer<{N * 4}>) {

//     }
// }

pub enum WordEndian {
    Big,
    Little,
}
// pub struct Words<T, const N: usize> {
//     _endian: PhantomData<WordEndian>,
//     _word: PhantomData<T>,
// }

// impl<u32, const N: usize> Words<u32, N> {
//     fn from_le_bytes(bytes: )
// }

// impl<const WORD_LEN: usize> From<[u8; 4 * WORD_LEN]> for Word<u32> {
//     fn from(value: [u8; core::mem::size_of::<T>() * WORD_LEN]) -> Self {

//     }
// }
