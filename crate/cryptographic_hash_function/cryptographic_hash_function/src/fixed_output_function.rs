use alloc::string::String;
use core::fmt::Write;

pub struct FixedOutput<const N: usize>([u8; N]);

impl<const N: usize> FixedOutput<N> {
    pub const fn new(digest: [u8; N]) -> Self {
        Self(digest)
    }
    pub const fn bytes(&self) -> [u8; N] {
        self.0
    }
    pub fn to_lowerhex(&self) -> String {
        let mut hex = String::with_capacity(N);
        for i in 0..N {
            let _ = write!(&mut hex, "{:02x}", self.0[i]);
        }
        hex
    }
    pub fn to_upperhex(&self) -> String {
        let mut hex = String::with_capacity(N);
        for i in 0..N {
            let _ = write!(&mut hex, "{:02X}", self.0[i]);
        }
        hex
    }
}

pub trait FixedOutputFunction<const N: usize> {
    fn update(&mut self, message: &[u8]);
    fn finalize(&mut self) -> FixedOutput<N>;
}
