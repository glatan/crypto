use crate::word::{Word, BigEndian};

pub struct Block<T, E, const N: usize> {
    pub words: [Word<T, E>; N],
}
