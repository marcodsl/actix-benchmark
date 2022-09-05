pub trait PrimeSolver {
    fn primes(&self) -> Vec<usize>;
}

pub struct Sieve {
    bound: usize,
}

impl Sieve {
    pub fn new(bound: usize) -> Self {
        Self { bound }
    }
}

impl PrimeSolver for Sieve {
    fn primes(&self) -> Vec<usize> {
        let mut primes: Vec<bool> = (0..self.bound + 1)
            .map(|num| num == 2 || num & 1 != 0)
            .collect();

        let mut num: usize = 3;

        while num * num <= self.bound {
            let mut index = num * num;
            while index <= self.bound {
                primes[index] = false;
                index += num;
            }

            num += 2;
        }

        primes
            .into_iter()
            .enumerate()
            .skip(2)
            .filter_map(|(index, prime)| prime.then(|| index))
            .collect()
    }
}
