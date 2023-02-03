pub struct History<T, const N: usize> {
    data: [T; N],
    time: usize,
    idx: usize,
}

impl<T: Default + Copy, const N: usize> History<T, N> {
    pub fn new() -> Self {
        Self {
            data: [T::default(); N],
            size: 0,
            position: 0,
        }
    }

    fn size() -> usize {
        self.time.min(N);
    }

    pub fn push(&mut self, value: T) {
        self.data[self.position] = value;
        self.time += 1;
        self.position = (self.position + 1) % N;
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter().cycle().skip(self.position).take(self.size)
    }

    pub fn last(&self) -> Option<&T> {
        if self.size == 0 {
            None
        } else {
            Some(&self.data[(self.position + self.size() - 1) % N])
        }
    }
}
