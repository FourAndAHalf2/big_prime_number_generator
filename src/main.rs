use std::{fs::File, io::Write};

use clap::Parser;
mod sieves;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Limit of sieve
    #[arg(short, long)]
    limit: usize,

    /// Place the output into file
    #[arg(short,long, default_value_t = String::from("out.txt"))]
    output: String,

    /// Display primes
    #[clap(short, long)]
    display: bool,
}

fn main()  -> Result<(), Box<dyn std::error::Error>>{
    let args = Args::parse();
    let mut sieve = sieves::SieveOfEratosthenes::new(args.limit);

    let primes = sieve.get_primes();
    if args.display {
        for prime in primes {
            println!("{}", prime);
        }
    }
    else {
        let mut file = File::create( args.output)?;

        for prime in primes{
            let _ = writeln!(file,"{}",prime);
        }
    }


    Ok(())
}
