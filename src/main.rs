use std::{fs::File, io::Write};


use clap::Parser;

use crate::{progress_bar::ProgressBar, settings::get_settings};
mod sieves;
mod tests; // without that line tests don't work
mod settings;
mod progress_bar;


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

    // hide progress bar
    #[clap(long)]
    hide: bool
}

fn main()  -> Result<(), Box<dyn std::error::Error>>{
    let args = Args::parse();

    get_settings().show_bar = !args.hide;

    let mut sieve = sieves::SieveOfEratosthenes::new(args.limit);

    let bar = ProgressBar::new(get_settings().show_bar);

    let primes = sieve.get_primes();
    if args.display {
        for prime in primes {
            println!("{}", prime);
        }
    }
    else {
        let mut file = File::create( args.output)?;

        let mut buffer = String::new();
        
        for prime in  bar.iter(primes){

            buffer +=  &format!("{}\n",prime);

            if buffer.len() > get_settings().buffor_size{
                let _ = write!(file,"{}",buffer);

                buffer.clear();
            }
           
        }
        if buffer.len() != 0 {
            let _ = write!(file,"{}",buffer);
        }
    }


    Ok(())
}
