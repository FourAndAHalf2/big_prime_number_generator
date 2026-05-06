// without this line there suggestion to remove next line - without it VSCode show errors in test.rs, but the program compiles
#![allow(unused)]
#![feature(test)]

use std::process;

use clap::{Parser, Subcommand};
use regex::Regex;

use crate::{
    settings::{get_settings, load_and_get_settings, load_settings},
    sieve_io::{BitSetSieveIO, SieveIO, TextSieveIO},
};
mod binary_array;
mod progress_bar;
mod settings;
mod sieve_io;
mod sieves;
mod tests;

use crate::sieves::Sieve;

#[derive(Subcommand, Debug)]
enum Commands {
    Read {
        /// file what is opened to read
        #[arg(short, long)]
        file: String,

        /// filter data what fit in the pattern, uses regex
        #[arg(long, default_value = "")]
        pattern: String,

        /// method of saving primes, available methods text, bitset, auto
        #[arg(short,long, default_value_t = load_and_get_settings().io_method.clone())]
        method: String,
    },
    Write {
        /// Limit of sieve
        #[arg(short, long)]
        limit: usize,

        /// Place the output into file
        #[arg(short, long, default_value_t = load_and_get_settings().output.clone())]
        output: String,

        /// Display primes
        #[arg(short, long)]
        display: bool,

        /// Hide progress bar
        #[arg(long)]
        hide: bool,

        /// Show progress bar
        #[arg(long)]
        show: bool,

        /// Type of sieve used to compute primes, available types: eratosthenes, atkin
        #[arg(short,long, default_value_t = load_and_get_settings().sieve_type.clone())]
        sieve: String,

        /// Size of buffer used for writing numbers
        #[arg(short,long, default_value_t = load_and_get_settings().buffer_size.clone())]
        buffer_size: usize,

        /// method of saving primes, available methods text, bitset
        #[arg(short,long, default_value_t = load_and_get_settings().io_method.clone())]
        method: String,
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
            show,
            sieve,
            buffer_size,
            method,
        }) => {
            if hide {
                get_settings().show_bar = false;
            }
            if show {
                get_settings().show_bar = true;
            }

            get_settings().buffer_size = buffer_size;

            let sieve_type = sieve;

            let mut sieve: Box<dyn Sieve> = match sieve_type.to_lowercase().as_str() {
                "eratosthenes" => Box::new(sieves::SieveOfEratosthenes::new(limit)),
                "atkin" => Box::new(sieves::SieveOfAtkin::new(limit)),
                _ => panic!("{} is not supported, use other algorithm", sieve_type),
            };

            let sieve_io = get_sieve_io(method);
            sieve.as_mut().run();

            if display {
                let primes = sieve.get_primes();
                for prime in primes {
                    println!("{}", prime);
                }
            } else {
                sieve_io.as_ref().write(sieve.get_sieve(), output)?;
            }
        }

        Some(Commands::Read {
            file: path,
            pattern,
            method: method_name,
        }) => {
            let re = Regex::new(&pattern);

            if re.is_err() {
                eprintln!("{} is invalid pattern", pattern);
                process::exit(1);
            }

            get_settings().io_method = method_name.clone();

            let re = re.unwrap();

    
            let mut primes: Option<Vec<usize>> = None;
            if method_name == "auto" {
                let aviable_io: Vec<Box<dyn SieveIO>> =
                    vec![Box::new(TextSieveIO), Box::new(BitSetSieveIO)];
                for method in aviable_io {
                    let status = method.as_ref().read(path.clone());
                    if status.is_err() {
                        continue;
                    }
                    primes = Some(status.unwrap())
                }
            } else {
                let result= get_sieve_io(method_name).as_ref().read(path);
                if result.is_err(){
                    eprintln!("file cannot be open with \"{}\" method ",get_settings().io_method);
                    process::exit(1);
                }
                primes = Some(result.unwrap())
            }

            if !primes.is_none() {
                for prime in primes.unwrap() {
                    if re.is_match(&format!("{}", prime)) {
                        println!("{}", prime)
                    }
                }
            }
            else {
                eprintln!("Invalid input file");
                process::exit(1)
            }
        }

        None => {
            eprintln!("No command provided. Use --help.");
            process::exit(1)
        }
    }

    Ok(())
}

fn get_sieve_io(method: String) -> Box<dyn SieveIO> {
    match method.to_lowercase().as_str() {
        "text" => Box::new(sieve_io::TextSieveIO),
        "bitset" => Box::new(sieve_io::BitSetSieveIO),
        _ => panic!("{}", method),
    }
}
