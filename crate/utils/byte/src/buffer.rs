use core::marker::PhantomData;

#[derive(Debug)]
pub struct Buffer<'a, const N: usize> {
    block: [u8; N],
    used_len: usize,
    lifetime: PhantomData<&'a ()>,
}

impl<'a, const N: usize> Buffer<'a, N> {
    pub const fn new() -> Self {
        Self {
            block: [0; N],
            used_len: 0,
            lifetime: PhantomData,
        }
    }
    pub fn fill_bytes(&mut self, input: &[u8]) -> usize {
        let use_input_len = if (self.used_len + input.len()) < N {
            input.len()
        } else {
            N - self.used_len
        };
        self.block[self.used_len..self.used_len + use_input_len]
            .copy_from_slice(&input[..use_input_len]);
        self.used_len += use_input_len;

        use_input_len
    }
    pub const fn full_filled(&self) -> bool {
        self.used_len == N
    }
    pub fn chunk(&mut self) -> [u8; N] {
        if self.used_len < N {
            panic!("Needs len: {}, current len: {}", N, self.used_len);
        }
        self.used_len = 0;
        self.block
    }
    pub fn reminder(&mut self) -> &[u8] {
        let used_len = self.used_len;
        self.used_len = 0;
        &self.block[..used_len]
    }
    pub fn take_blocks_with_input(&mut self, input: &[u8]) -> Option<([u8; N], usize)> {
        if self.used_len + input.len() < N {
            return None;
        }

        let mut block = [0; N];
        for i in 0..(N - self.used_len) {
            block[i + self.used_len] = input[i];
        }
        self.used_len = 0;
        Some((block, N - self.used_len))
    }
}

#[cfg(test)]
mod tests {
    use super::Buffer;

    #[test]
    fn fill_less_than_max_len<'a>() {
        let mut buffer: Buffer<'a, 4> = Buffer::new();
        buffer.fill_bytes(&[u8::MAX; 1]);
        assert_eq!(buffer.used_len, 1);
        assert_eq!(buffer.block[0], u8::MAX);
        assert_eq!(buffer.block[1], 0);
        assert_eq!(buffer.block[2], 0);
        assert_eq!(buffer.block[3], 0);
    }

    #[test]
    fn fill_equals_max_len<'a>() {
        let mut buffer: Buffer<'a, 4> = Buffer::new();
        buffer.fill_bytes(&[u8::MAX; 4]);
        assert_eq!(buffer.used_len, 4);
        assert_eq!(buffer.block[0], u8::MAX);
        assert_eq!(buffer.block[1], u8::MAX);
        assert_eq!(buffer.block[2], u8::MAX);
        assert_eq!(buffer.block[3], u8::MAX);
    }

    #[test]
    fn fill_greater_than_max_len<'a>() {
        let mut buffer: Buffer<'a, 4> = Buffer::new();
        buffer.fill_bytes(&[u8::MAX; 4]);
        assert_eq!(buffer.used_len, 4);
        assert_eq!(buffer.block[0], u8::MAX);
        assert_eq!(buffer.block[1], u8::MAX);
        assert_eq!(buffer.block[2], u8::MAX);
        assert_eq!(buffer.block[3], u8::MAX);
    }

    #[test]
    fn take_all<'a>() {
        let mut buffer: Buffer<'a, 4> = Buffer::new();
        buffer.fill_bytes(&[u8::MAX; 4]);
        let result = buffer.take_all();
        assert_eq!(result, buffer.block);
        assert_eq!(buffer.used_len, 0);
    }

    #[test]
    fn reminder<'a>() {
        let mut buffer: Buffer<'a, 4> = Buffer::new();
        buffer.fill_bytes(&[u8::MAX; 2]);
        let result = buffer.reminder();
        assert_eq!(result, &[u8::MAX; 2]);
        assert_eq!(*&buffer.used_len, 0);
    }

    #[test]
    fn full_filled<'a>() {
        let mut buffer: Buffer<'a, 8> = Buffer::new();
        buffer.fill_bytes(&[0; 8]);
        assert!(buffer.full_filled());
    }
}
