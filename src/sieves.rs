use crate::{binnary_array::BinaryArray, progress_bar::ProgressBar, settings::get_settings};


pub trait Sieve {
    fn get_limit(&self) -> usize;
    #[allow(unused)]
    fn set_limit(&mut self, new_limit: usize);

    fn run(&mut self);
    fn get_primes(&mut self) -> Vec<usize>;
    
}


pub struct SieveOfEratosthenes {
    sieve: BinaryArray,
    is_sieve_completed: bool,
}

impl SieveOfEratosthenes {
    pub fn new(limit: usize) -> SieveOfEratosthenes {
        return SieveOfEratosthenes {
            sieve: BinaryArray::new(limit + 1, true),
            is_sieve_completed: false,
        };
    }
}

impl Sieve for SieveOfEratosthenes {
    fn get_limit(&self) -> usize {
        return self.sieve.len() - 1;
    }

    #[allow(unused)]
    fn set_limit(&mut self, new_limit: usize) {
        self.sieve = BinaryArray::new(new_limit + 1, true);
        self.is_sieve_completed = false;
    }

    fn run(&mut self) {
        let bar = ProgressBar::new(get_settings().show_bar);

        let limit = (self.get_limit() as f32 + 1.0).sqrt() as usize+1;

        self.sieve[0] = false;
        self.sieve[1] = false;
        for i in bar.iter(1..=limit) {
            if self.sieve[i] {
                for j in (i * i..=self.get_limit()).step_by(i) {
                    self.sieve[j] = false;
                }
            }
        }
        self.is_sieve_completed = true;
    }

    fn get_primes(&mut self) -> Vec<usize> {
        let bar = ProgressBar::new(get_settings().show_bar);

        if !self.is_sieve_completed {
            self.run();
        }

        let mut primes = vec![];

        for i in bar.iter(0..=self.get_limit()) {
            if self.sieve[i] {
                primes.push(i);
            }
        }

        return primes;
    }
}

pub struct SieveOfAtkin {
    sieve: BinaryArray,
    is_sieve_completed: bool,
}

impl SieveOfAtkin {
    pub fn new(limit: usize) -> SieveOfAtkin {
        SieveOfAtkin {
            sieve: BinaryArray::new(limit + 1, false),
            is_sieve_completed: false,
        }
    }
}

impl Sieve for SieveOfAtkin {
    fn get_limit(&self) -> usize {
        self.sieve.len() - 1
    }

    fn set_limit(&mut self, new_limit: usize) {
        self.sieve = BinaryArray::new(new_limit + 1, false);
        self.is_sieve_completed = false;
    }

     fn run(&mut self) {
        let bar = ProgressBar::new(get_settings().show_bar);

        if self.get_limit() > 2 {
            self.sieve[2] = true;
            // self.sieve.set(2, true);
        }
        if self.get_limit() > 3 {
            self.sieve[3] = true;
            // self.sieve.set(3, true);
        }

        let limit_sqrt = (self.get_limit() as f64).sqrt() as usize;

        for x in  bar.iter(1..=limit_sqrt) {
            for y in 1..=limit_sqrt {
                let x_square = x * x;
                let y_square = y * y;

                let n = (x_square << 2) + y_square;
                if (n <= self.get_limit()) & (n % 12 == 1 || n % 12 == 5) {
                    self.sieve[n] = !self.sieve[n]
                    // self.sieve.set(n, ((self.sieve[n] as usize + 1) % 2) != 0);
                }

                let n = (3 * x_square) + (y_square);
                if (n <= self.get_limit()) & (n % 12 == 7) {
                    // self.sieve[n] ^= 1;
                    self.sieve[n] = !self.sieve[n]
                }

                if (3 * x_square) > (y_square) {
                    let n = (3 * x * x) - (y * y);
                    if (x > y) & (n <= self.get_limit()) & (n % 12 == 11) {
                        self.sieve[n] = !self.sieve[n]
                    }
                }
            }
        }

        for i in bar.iter(5..=limit_sqrt) {
            if i * i > self.get_limit() {
                break;
            }
            if self.sieve[i] == false {
                continue;
            }
            for j in (i * i..=self.get_limit()).step_by(i * i) {
                // self.sieve.set(j, false);
                self.sieve[j] = false;
            }
        }

        self.is_sieve_completed = true;
    }

    fn get_primes(&mut self) -> Vec<usize> {
        let bar = ProgressBar::new(get_settings().show_bar);

        if !self.is_sieve_completed {
            self.run();
        }

        let mut primes = vec![];

        for i in bar.iter(2..=self.get_limit()) {
            if self.sieve[i] {
                primes.push(i);
            }
        }

        primes
    }
}