use std::{fs::File, io::BufReader};
use std::io::Write;
use std::io::BufRead;

use regex::Regex;

use crate::{binary_array::BinaryArray, progress_bar::ProgressBar, settings::get_settings};

pub trait SieveIO {
    fn write(&self, sieve: &BinaryArray, path: String) -> Result<(), Box<dyn std::error::Error>>;

    fn read(&self, path: String, pattern: String) ->  Result<Vec<usize>, Box<dyn std::error::Error>>;
}

pub struct TextSieveIO;

impl SieveIO for TextSieveIO {
    fn write(&self, sieve: &BinaryArray, path: String) -> Result<(), Box<dyn std::error::Error>> {
        let bar = ProgressBar::new(get_settings().show_bar);

        let mut file = File::create(path)?;
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

    fn read(&self, path: String,pattern: String) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        
        let mut primes = vec![];
        for line in reader.lines() {
            let line = line?;
            primes.push(line.parse().unwrap());
        };

        Ok(primes)
    }
}


