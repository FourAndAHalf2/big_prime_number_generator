pub struct SieveOfEratosthenes {
    sieve: Vec<bool>,
    is_sieve_completed: bool,
}

impl SieveOfEratosthenes {
    pub fn new(limit: usize) -> SieveOfEratosthenes {
        return SieveOfEratosthenes {
            sieve: vec![true; limit + 1 as usize],
            is_sieve_completed: false,
        };
    }

    pub fn get_limit(&self) -> usize {
        return self.sieve.len() - 1;
    }

    #[allow(unused)]
    pub fn set_limit(&mut self, new_limit: usize) {
        self.sieve = vec![true; new_limit + 1];
        self.is_sieve_completed = false;
    }

    pub fn run(&mut self) {
        let limit = (self.get_limit() as f32 + 1.0).sqrt() as usize;

        self.sieve[0] = false;
        self.sieve[1] = false;
        for i in 1..=limit {
            if self.sieve[i] {
                for j in (2 * i..=self.get_limit()).step_by(i) {
                    self.sieve[j] = false;
                }
            }
        }
        self.is_sieve_completed = true;
    }

    pub fn get_primes(&mut self) -> Vec<usize> {
        if !self.is_sieve_completed {
            self.run();
        }

        let mut primes = vec![];

        for i in 0..=self.get_limit() {
            if self.sieve[i] {
                primes.push(i);
            }
        }

        return primes;
    }
}
