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

    /// Create a random particle
    fn size(&self) -> usize {
        self.time.max(N)
    }

    fn idx(&self) -> usize {
        (self.base + self.size()) % N
    }

    pub fn push(&mut self, value: (f64, f64)) {
        self.data[self.idx()] = value;
        self.time += 1;
        if self.time >= N {
            self.base = (self.base + 1) % N;
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &(f64, f64)> {
        self.data.iter().cycle().skip(self.base).take(self.size())
    }

    pub fn nth(&self, n: usize) -> Option<(f64, f64)> {
        if n <= self.size() {
            Some(self.data[(self.base + n) % N])
        } else {
            None
        }
    }

    pub fn last(&self) -> Option<(f64, f64)> {
        if self.size() != 0 {
            Some(self.data[self.idx() - 1])
        } else {
            None
        }
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

    pub fn push(&mut self, pos: (f64, f64)) {
        if self.data.is_empty() {
            self.data.push(Particle::new());
        }
        self.data.last_mut().unwrap().push(pos);
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
                    |particle| match (particle.nth(self.i), particle.nth(self.i + 1)) {
                        (Some((x0, y0)), Some((x1, y1))) => Some([x0, y0, x1, y1]),
                        _ => None,
                    },
                )
                .collect();

            Some(item)
        } else {
            None
        }
    }
}
