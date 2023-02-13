use rand::random;

pub struct Particle<const N: usize> {
    data: [(f64, f64); N],
    time: usize,
    base: usize,
}

impl<const N: usize> Particle<N> {
    pub fn new() -> Self {
        Self {
            data: [(0.0, 0.0); N],
            time: 0,
            base: 0,
        }
    }

    /// Create a random particle
    pub fn random() -> Self {
        let mut this = Self::new();
        this.push((random(), random()));
        this
    }

    /// Return the size of the history of the particle
    fn size(&self) -> usize {
        self.time.min(N)
    }

    /// Index of the next available space
    fn idx(&self) -> usize {
        (self.base + self.size()) % N
    }

    pub fn push(&mut self, value: (f64, f64)) {
        self.data[self.idx()] = value;
        self.time += 1;
        if self.time > N {
            self.base = (self.base + 1) % N;
        }
    }

    pub fn _iter(&self) -> impl Iterator<Item = &(f64, f64)> {
        self.data.iter().cycle().skip(self.base).take(self.size())
    }

    pub fn at(&self, n: usize) -> Option<(f64, f64)> {
        debug_assert!(n<N);
        if n < self.size() {
            Some(self.data[(self.base + n) % N])
        } else {
            None
        }
    }

    /// Return the last element of the history of the particle
    pub fn last(&self) -> Option<(f64, f64)> {
        if self.size() != 0 {
            Some(self.data[(self.idx() + N - 1) % N])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test_particle {
    use crate::history::Particle;

    #[test]
    fn time() {
        let mut particle = Particle::<5>::new();
        assert_eq!(particle.time, 0);

        particle.push((1.0, 1.0));
        particle.push((2.0, 2.0));
        particle.push((3.0, 3.0));
        assert_eq!(particle.time, 3);

        particle.push((4.0, 4.0));
        particle.push((5.0, 5.0));
        particle.push((6.0, 6.0));
        assert_eq!(particle.time, 6);
    }

    #[test]
    fn size() {
        let mut particle = Particle::<5>::new();
        assert_eq!(particle.size(), 0);

        particle.push((1.0, 1.0));
        particle.push((2.0, 2.0));
        particle.push((3.0, 3.0));
        assert_eq!(particle.size(), 3);

        particle.push((4.0, 4.0));
        particle.push((5.0, 5.0));
        particle.push((6.0, 6.0));
        assert_eq!(particle.size(), 5);
    }

    #[test]
    fn last() {
        let mut particle = Particle::<5>::new();
        assert_eq!(particle.last(), None);

        particle.push((1.0, 1.0));
        particle.push((2.0, 2.0));
        particle.push((3.0, 3.0));
        assert_eq!(particle.last(), Some((3.0, 3.0)));

        particle.push((4.0, 4.0));
        particle.push((5.0, 5.0));
        particle.push((6.0, 6.0));
        assert_eq!(particle.last(), Some((6.0, 6.0)));
    }

    #[test]
    fn at() {
        let mut particle = Particle::<5>::new();
        assert_eq!(particle.at(0), None);
        assert_eq!(particle.at(4), None);

        particle.push((1.0, 1.0));
        particle.push((2.0, 2.0));
        particle.push((3.0, 3.0));
        assert_eq!(particle.at(0), Some((1.0, 1.0)));
        assert_eq!(particle.at(4), None);

        particle.push((4.0, 4.0));
        particle.push((5.0, 5.0));
        particle.push((6.0, 6.0));
        assert_eq!(particle.at(0), Some((2.0, 2.0)));
        assert_eq!(particle.at(4), Some((6.0, 6.0)));
    }
}

pub struct History<const N: usize> {
    data: Vec<Particle<N>>,
}

impl<const N: usize> History<N> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    // Insert a new particle in history with a random position
    pub fn spawn(&mut self) {
        self.data.push(Particle::<N>::random());
    }

    /// Go the history by generation (Iterators with the same time)
    pub fn gen_iter<'a>(&'a self) -> impl Iterator<Item = Vec<[f64; 4]>> + 'a {
        HistoryIterator {
            history: self,
            i: 0,
        }
    }

    pub fn data_iter_mut(&mut self) -> impl Iterator<Item = &mut Particle<N>> {
        self.data.iter_mut()
    }
}

// History Iterator
struct HistoryIterator<'a, const N: usize> {
    history: &'a History<N>,
    i: usize,
}

impl<'a, const N: usize> Iterator for HistoryIterator<'a, N> {
    type Item = Vec<[f64; 4]>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i + 1 < N {
            let item = self
                .history
                .data
                .iter()
                .filter_map(
                    |particle| match (particle.at(self.i), particle.at(self.i + 1)) {
                        (Some((x0, y0)), Some((x1, y1))) => Some([x0, y0, x1, y1]),
                        _ => None,
                    },
                )
                .collect();

            self.i += 1;
            Some(item)
        } else {
            None
        }
    }
}
