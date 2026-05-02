#![feature(test)]
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};

use clap::{Parser, Subcommand};

use crate::{
    progress_bar::ProgressBar,
    settings::{get_settings, load_settings},
};
mod binnary_array;
mod progress_bar;
mod settings;
mod sieves;
mod tests;
use crate::sieves::Sieve;

#[derive(Subcommand, Debug)]
enum Commands {
    Read {
        /// file what is opened to read
        #[arg(short, long)]
        file: String,
    },
    Write {
        /// Limit of sieve
        #[arg(short, long)]
        limit: usize,

        /// Place the output into file
        #[arg(short, long, default_value_t = String::from("out.txt"))]
        output: String,

        /// Display primes
        #[arg(short, long)]
        display: bool,

        /// Hide progress bar
        #[arg(long)]
        hide: bool,

        /// type of sieve used to compute primes, aviable types eratosthenes, atkin
        #[arg(short,long, default_value_t = String::from("eratosthenes"))]
        sieve: String
    },
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let _ = load_settings();

    match args.command {
        Some(Commands::Write {
            limit,
            output,
            display,
            hide,
            sieve
        }) => {
            get_settings().show_bar = !hide;

            let sieve_type = sieve;
            let mut sieve: Box<dyn Sieve> = Box::new(sieves::SieveOfEratosthenes::new(limit));

            if sieve_type == "atkin" {
                sieve = Box::new(sieves::SieveOfAtkin::new(limit));
            }

            let bar = ProgressBar::new(get_settings().show_bar);

            let primes = sieve.get_primes();

            if display {
                for prime in primes {
                    println!("{}", prime);
                }
            } else {
                let mut file = File::create(output)?;
                let mut buffer = String::new();

                for prime in bar.iter(primes) {
                    buffer += &format!("{}\n", prime);

                    if buffer.len() > get_settings().buffer_size {
                        write!(file, "{}", buffer)?;
                        buffer.clear();
                    }
                }

                if !buffer.is_empty() {
                    write!(file, "{}", buffer)?;
                }
            }
        }

        Some(Commands::Read { file: path }) => {
            let file = File::open(path)?;
            let reader = BufReader::new(file);

            for line in reader.lines() {
                println!("{}", line?);
            }
        }

        None => {
            println!("No command provided. Use --help.");
        }
    }

    Ok(())
}
