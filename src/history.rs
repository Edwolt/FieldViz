pub struct History<T, const N: usize> {
    data: [T; N],
    pub time: usize, // TODO make private
    idx: usize,
}

impl<T: Default + Copy, const N: usize> History<T, N> {
    pub fn new() -> Self {
        Self {
            data: [T::default(); N],
            time: 0,
            idx: 0,
        }
    }

    fn size(&self) -> usize {
        self.time.min(N)
    }

    pub fn push(&mut self, value: T) {
        self.data[self.idx] = value;
        self.time += 1;
        self.idx = (self.idx + 1) % N;
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter().cycle().skip(self.idx).take(self.size())
    }

    pub fn last(&self) -> Option<&T> {
        if self.size() == 0 {
            None
        } else {
            Some(&self.data[(self.idx + N - 1) % N])
        }
    }
}
