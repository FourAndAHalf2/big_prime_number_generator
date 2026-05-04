// without this line there suggestion to remove next line - without it VSCode show errors in test.rs, but the program compiles
#![allow(unused)]
#![feature(test)]

use clap::{Parser, Subcommand};

use crate::{
    settings::{get_settings, load_and_get_settings, load_settings},
    sieve_io::{SieveIO, TextSieveIO},
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

        /// Type of sieve used to compute primes, aviable types eratosthenes, atkin
        #[arg(short,long, default_value_t = load_and_get_settings().sieve_type.clone())]
        sieve: String,

        /// Size of buffer used for writing numbers
        #[arg(short,long, default_value_t = load_and_get_settings().buffer_size.clone())]
        buffer_size: usize,
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

            sieve.as_mut().run();

            if display {
                let primes = sieve.get_primes();
                for prime in primes {
                    println!("{}", prime);
                }
            } else {
                TextSieveIO {}.write(sieve.get_sieve(), output);
            }
        }

        Some(Commands::Read {
            file: path,
            pattern,
        }) => {
            TextSieveIO {}.read(path, pattern)?;
        }

        None => {
            println!("No command provided. Use --help.");
        }
    }

    Ok(())
}
