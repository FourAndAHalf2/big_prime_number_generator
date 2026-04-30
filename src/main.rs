use std::{fs::{File, read_to_string}, io::Write};

use clap::{Parser, Subcommand};

use crate::{progress_bar::ProgressBar, settings::{get_settings, load_settings}};
mod progress_bar;
mod settings;
mod sieves;
mod tests;

#[derive(Subcommand, Debug)]
enum Commands {
    Read {
        /// file what is opened to read
        #[arg(short,long)]
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


   
    match args.command {
        Some(Commands::Write {
            limit,
            output,
            display,
            hide,
        }) => {

            let _ = load_settings();
            get_settings().show_bar = !hide;

            let mut sieve = sieves::SieveOfEratosthenes::new(limit);
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
            for line in read_to_string(path)?.lines(){
                println!("{}",line);
            }
        }


        None => {
            println!("No command provided. Use --help.");
        }
    }

    Ok(())
}