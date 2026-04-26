use clap::Parser;
mod sieves;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Limit of sieve
    #[arg(short, long)]
    limit: usize,
}

fn main() {
    let args = Args::parse();
    let mut sieve = sieves::SieveOfEratosthenes::new(args.limit);
   
   let primes = sieve.get_primes();

   for prime in primes{
    println!("{}",prime)
   }
}