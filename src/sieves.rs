use std::fs::File;
use std::io::Write;

use crate::{binary_array::BinaryArray, progress_bar::ProgressBar, settings::get_settings};

pub trait Sieve {
    fn get_sieve(&self) -> &BinaryArray;
    fn get_sieve_mut(&mut self) -> &mut BinaryArray;

    fn get_limit(&self) -> usize {
        self.get_sieve().len() - 1
    }
    #[allow(unused)]
    fn set_limit(&mut self, new_limit: usize);

    fn run(&mut self);

   fn save(&mut self, output: String) -> Result<(), Box<dyn std::error::Error>> {
        self.run();
        save_sieve(&self.get_sieve(), output)
    }

    fn get_primes(&mut self) -> Vec<usize> {
        self.run();

        
        let bar = ProgressBar::new(get_settings().show_bar);

        let mut primes = Vec::new();

        for i in bar.iter(2..=self.get_limit()) {
            if self.get_sieve()[i] {
                primes.push(i);
            }
        }

        primes
    }
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

fn save_sieve(sieve: &BinaryArray, output: String) -> Result<(), Box<dyn std::error::Error>> {
    let bar = ProgressBar::new(get_settings().show_bar);

    let mut file = File::create(output)?;
    let mut buffer = String::new();

    for i in bar.iter(0..sieve.len()) {
        let is_prime = sieve[i];

        if !is_prime {
            continue;
        }
        buffer += &format!("{}\n", i);

        if buffer.len() > get_settings().buffer_size {
            write!(file, "{}", buffer)?;
            buffer.clear();
        }
    }

    if !buffer.is_empty() {
        write!(file, "{}", buffer)?;
    }
    Ok(())
}

impl Sieve for SieveOfEratosthenes {
    fn get_sieve(&self) -> &BinaryArray {
        &self.sieve
    }

    fn get_sieve_mut(&mut self) -> &mut BinaryArray {
        &mut self.sieve
    }

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

        let limit = (self.get_limit() as f32 + 1.0).sqrt() as usize + 1;

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
    fn get_sieve(&self) -> &BinaryArray {
        &self.sieve
    }

    fn get_sieve_mut(&mut self) -> &mut BinaryArray {
        &mut self.sieve
    }

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
        }
        if self.get_limit() > 3 {
            self.sieve[3] = true;
        }

        let limit_sqrt = (self.get_limit() as f64).sqrt() as usize;

        for x in bar.iter(1..=limit_sqrt) {
            for y in 1..=limit_sqrt {
                let x_square = x * x;
                let y_square = y * y;

                let n = (x_square << 2) + y_square;
                if (n <= self.get_limit()) & (n % 12 == 1 || n % 12 == 5) {
                    self.sieve[n] = !self.sieve[n]
                }

                let n = (3 * x_square) + (y_square);
                if (n <= self.get_limit()) & (n % 12 == 7) {
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
                self.sieve[j] = false;
            }
        }

        self.is_sieve_completed = true;
    }

    
}
