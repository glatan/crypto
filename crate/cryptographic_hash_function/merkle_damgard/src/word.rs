use core::marker::PhantomData;

pub trait EndianTransform: Sized + Copy {
    type Bytes;
    fn to_be_bytes(self) -> Self::Bytes;
    fn to_le_bytes(self) -> Self::Bytes;
    fn from_be_bytes(bytes: Self::Bytes) -> Self;
    fn from_le_bytes(bytes: Self::Bytes) -> Self;
}

impl EndianTransform for u32 {
    type Bytes = [u8; 4];
    fn to_be_bytes(self) -> Self::Bytes { self.to_be_bytes() }
    fn to_le_bytes(self) -> Self::Bytes { self.to_le_bytes() }
    fn from_be_bytes(bytes: Self::Bytes) -> Self { u32::from_be_bytes(bytes) }
    fn from_le_bytes(bytes: Self::Bytes) -> Self { u32::from_le_bytes(bytes) }
}

impl EndianTransform for u64 {
    type Bytes = [u8; 8];
    fn to_be_bytes(self) -> Self::Bytes { self.to_be_bytes() }
    fn to_le_bytes(self) -> Self::Bytes { self.to_le_bytes() }
    fn from_be_bytes(bytes: Self::Bytes) -> Self { u64::from_be_bytes(bytes) }
    fn from_le_bytes(bytes: Self::Bytes) -> Self { u64::from_le_bytes(bytes) }
}

pub trait Endian {
    fn to_bytes<T: EndianTransform>(word: T) -> T::Bytes;
    fn from_bytes<T: EndianTransform>(bytes: T::Bytes) -> T;
}

pub struct BigEndian;
impl Endian for BigEndian {
    fn to_bytes<T: EndianTransform>(word: T) -> T::Bytes { word.to_be_bytes() }
    fn from_bytes<T: EndianTransform>(bytes: T::Bytes) -> T { T::from_be_bytes(bytes) }
}

pub struct LittleEndian;
impl Endian for LittleEndian {
    fn to_bytes<T: EndianTransform>(word: T) -> T::Bytes { word.to_le_bytes() }
    fn from_bytes<T: EndianTransform>(bytes: T::Bytes) -> T { T::from_le_bytes(bytes) }
}

#[derive(Debug, Clone, Copy)]
pub struct Word<T, E> {
    pub value: T,
    _endian: PhantomData<E>,
}

impl<T: EndianTransform, E: Endian> Word<T, E> {
    pub fn new(value: T) -> Self {
        Self { value, _endian: PhantomData }
    }

    pub fn to_bytes(self) -> T::Bytes {
        E::to_bytes(self.value)
    }

    pub fn from_bytes(bytes: T::Bytes) -> Self {
        Self::new(E::from_bytes(bytes))
    }
}
