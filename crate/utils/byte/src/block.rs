// use core::marker::PhantomData;

// pub enum WordEndian {
//     Big,
//     Little,
// }

// pub struct Block<T: Sized, const N: usize>{
//     words: [T; N],
//     _endian: PhantomData<WordEndian>
// }

// impl<const N: usize> Block<u32, N> {
//     pub fn word_len() -> usize {
//         N
//     }
//     pub fn from_bytes(bytes: &[u8; Self::word_len() * 8]) -> Self {
//         unimplemented!();
//     }
// }
