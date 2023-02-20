use rand::random;
use std::collections::VecDeque;

pub struct Particle<const N: usize> {
    data: VecDeque<(f64, f64)>,
    time: u32,
    pub valid: bool, 
}

impl<const N: usize> Particle<N> {
    pub fn new() -> Self {
        Self {
            data: VecDeque::new(),
            time: 0,
            valid: true,
        }
    }

    /// Create a random particle
    pub fn random() -> Self {
        let mut this = Self::new();

        let (x, y) = (random::<f64>(), random::<f64>());
        let (x, y) = (x * 2.0 - 1.0, y * 2.0 - 1.0);
        // Particle can appear outside of the canvas
        let (x, y) = (x * 1.25, y * 1.25);

        this.push((x, y));
        this
    }

    /// Return the size of the history of the particle
    fn len(&self) -> usize {
        return self.data.len();
    }

    // TODO can be transformed in update
    // Receiving the Filed function
    pub fn push(&mut self, value: (f64, f64)) {
        if self.valid {
            self.data.push_back(value);
            if self.data.len() > N {
                self.data.pop_front();
            }
            self.time += 1;
        } else {
            self.data.pop_front();
        }
    }

    pub fn _iter(&self) -> impl Iterator<Item = &(f64, f64)> {
        self.data.iter()
    }

    pub fn at(&self, n: usize) -> Option<(f64, f64)> {
        assert!(n < N);
        if n < self.len() {
            Some(self.data[n])
        } else {
            None
        }
    }

    /// Return the last element of the history of the particle
    pub fn last(&self) -> Option<&(f64, f64)> {
        self.data.back()
    }
}

#[cfg(test)]
mod tests_particle {
    use super::Particle;

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
        assert_eq!(particle.len(), 0);

        particle.push((1.0, 1.0));
        particle.push((2.0, 2.0));
        particle.push((3.0, 3.0));
        assert_eq!(particle.len(), 3);

        particle.push((4.0, 4.0));
        particle.push((5.0, 5.0));
        particle.push((6.0, 6.0));
        assert_eq!(particle.len(), 5);
    }

    #[test]
    fn last() {
        let mut particle = Particle::<5>::new();
        assert_eq!(particle.last(), None);

        particle.push((1.0, 1.0));
        particle.push((2.0, 2.0));
        particle.push((3.0, 3.0));
        assert_eq!(particle.last(), Some(&(3.0, 3.0)));

        particle.push((4.0, 4.0));
        particle.push((5.0, 5.0));
        particle.push((6.0, 6.0));
        assert_eq!(particle.last(), Some(&(6.0, 6.0)));
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

// TODO history should be linked to dt
// not to number of states stored
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

    // Remove particles that are older than expiration_date
    pub fn expires(&mut self, expiration_date: u32) {
        self.data.iter_mut().for_each(|i| {
            if i.time > expiration_date {
                i.valid = false;
            }
        });

        self.data.retain(|p| p.valid && p.len() != 0);
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

#[cfg(test)]
mod tests_history {
    use super::History;
    use super::Particle;

    #[test]
    fn gen_iter() {
        let history = {
            let mut history = History::<5>::new();
            let vals = [0.0, 1.0, 2.0, 3.0, 4.0];

            for y in vals {
                let mut particle = Particle::<5>::new();
                for x in vals {
                    particle.push((x, y))
                }
                history.data.push(particle)
            }
            history
        };

        let mut iter = history.gen_iter();

        assert_eq!(
            iter.next(),
            Some(vec![
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 1.0, 1.0, 1.0],
                [0.0, 2.0, 1.0, 2.0],
                [0.0, 3.0, 1.0, 3.0],
                [0.0, 4.0, 1.0, 4.0],
            ])
        );

        assert_eq!(
            iter.next(),
            Some(vec![
                [1.0, 0.0, 2.0, 0.0],
                [1.0, 1.0, 2.0, 1.0],
                [1.0, 2.0, 2.0, 2.0],
                [1.0, 3.0, 2.0, 3.0],
                [1.0, 4.0, 2.0, 4.0],
            ])
        );
    }
}
