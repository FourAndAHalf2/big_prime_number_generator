use std::io::BufRead;
use std::io::Read;
use std::io::Write;
use std::{fs::File, io::BufReader};

use crate::{binary_array::BinaryArray, progress_bar::ProgressBar, settings::get_settings};

pub trait SieveIO {
    fn write(&self, sieve: &BinaryArray, path: String) -> Result<(), Box<dyn std::error::Error>>;

    fn read(&self, path: String) -> Result<Vec<usize>, Box<dyn std::error::Error>>;
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

    fn read(&self, path: String) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut primes = vec![];
        for line in reader.lines() {
            let line = line?;
            primes.push(line.parse()?);
        }

        Ok(primes)
    }
}

pub struct BitSetSieveIO;

impl SieveIO for BitSetSieveIO {
    fn write(&self, sieve: &BinaryArray, path: String) -> Result<(), Box<dyn std::error::Error>> {
        let bar = ProgressBar::new(get_settings().show_bar);

        let mut file = File::create(path)?;

        write!(file, "bits")?; // signature for this format

        let padding = if sieve.len() % 64 == 0 {
            0
        } else {
            64 - (sieve.len() % 64)
        };

        file.write_all(&[padding as u8])?; // count of unused bits in the last chunk

        let chunk_count = (sieve.len() + 63) / 64;

        let mut buffer = vec![];
        let buffer_max_size = get_settings().buffer_size;

        for i in bar.iter(0..chunk_count) {
            let chunk = sieve.get_chunk(i);
            buffer.extend_from_slice(&chunk.to_be_bytes());

            if buffer.len() > buffer_max_size{
                file.write(&buffer)?;
                buffer.clear();
            } 
        }

         if buffer.len() > 0{
                file.write(&buffer)?;
                
            } 
        Ok(())
    }

    fn read(&self, path: String) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
        let mut file = File::open(path)?;

        let mut file_signature = [0u8; 4];

        file.read_exact(&mut file_signature)?;
        if &file_signature != b"bits" {
            return Err(format!(
                "invalid file signature: expected \"bits\", got {:?}",
                file_signature
            )
            .into());
        }

        let mut padding_buffer = [0u8; 1];
        file.read_exact(&mut padding_buffer)?;
        let padding_size = padding_buffer[0] as usize;

        if padding_size >= 64 {
            return Err(format!("invalid padding value {}", padding_size).into());
            // it's unlikly to run this line
        }

        let metadata = file.metadata()?;
        let file_size: usize = metadata.len() as usize - ("bits".len() + 1);
        if file_size % 8 != 0 {
            return Err("invalid bitset payload size".into());
        }

        let chunk_count = file_size / 8;
        let total_length = chunk_count * 64 - padding_size;

        
        let mut chunk_buf = [0u8; 8];
        let mut primes = Vec::new();
        let bar = ProgressBar::new(get_settings().show_bar);

        for chunk_index in bar.iter(0..chunk_count) {
            file.read_exact(&mut chunk_buf)?;
            let chunk = u64::from_be_bytes(chunk_buf);

            let base = chunk_index * 64;
            for bit in 0..64 {
                let index = base + bit;
                if index >= total_length {
                    break;
                }
                if chunk & (1 << (63 - bit)) != 0 {
                    primes.push(index);
                }
            }
        }

        Ok(primes)
    }
}
