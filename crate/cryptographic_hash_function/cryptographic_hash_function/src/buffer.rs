#[derive(Clone, Copy)]
pub struct Buffer<const N: usize> {
    data: [u8; N],
    len: usize,
}

// is_emptyは使わない
#[allow(clippy::len_without_is_empty)]
impl<const N: usize> Buffer<N> {
    pub const fn new() -> Self {
        Self {
            data: [0; N],
            len: 0,
        }
    }

    /// Write data into the buffer.
    /// Returns the number of bytes written.
    pub fn write(&mut self, data: &[u8]) -> usize {
        if data.is_empty() {
            return 0;
        }

        let len = data.len().min(N - self.len);
        self.data[self.len..self.len + len].copy_from_slice(&data[..len]);
        self.len += len;
        len
    }

    pub fn fill_remaining(&mut self, value: u8) {
        self.data[self.len..].fill(value);
        self.len = N;
    }

    pub const fn is_full(&self) -> bool {
        self.len == N
    }

    pub const fn get_all(&self) -> &[u8; N] {
        &self.data
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn clear(&mut self) {
        self.data = [0; N];
        self.len = 0;
    }
}

impl<const N: usize> Default for Buffer<N> {
    fn default() -> Self {
        Self::new()
    }
}
