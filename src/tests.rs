#[cfg(test)]
mod tests {
    extern crate test;

    use crate::{
        binnary_array::{self, BinaryArray},
        settings::get_settings,
        sieves::{Sieve, SieveOfAtkin, SieveOfEratosthenes},
    };
    use test::Bencher;

    fn test_sieve<F>(create_sieve: F, limit: usize, expected_primes: Vec<usize>)
    where
        F: Fn(usize) -> Box<dyn Sieve>,
    {
        get_settings().show_bar = false;
        let mut sieve = create_sieve(limit);
        let primes = sieve.get_primes();
        
        assert_eq!(primes, expected_primes);
    }


    #[test]
    fn test_all_sieves() {
        let sieve_creators: Vec<Box<dyn Fn(usize) -> Box<dyn Sieve>>> = vec![
            Box::new(|limit| Box::new(SieveOfEratosthenes::new(limit))),
            Box::new(|limit| Box::new(SieveOfAtkin::new(limit))),
        ];

        let expected_primes = vec![2, 3, 5, 7];
        for creator in sieve_creators {
            test_sieve(creator, 10, expected_primes.clone());
        }
    }

    #[test]
    fn test_binary_array_read() {
        let data = binnary_array::BinaryArray::new(8, false);
        assert_eq!(data[0], false);
        assert_eq!(data[7], false);
    }

    #[test]
    fn test_binary_array_write() {
        let mut data = binnary_array::BinaryArray::new(10, false);
        data[7] = true;
        assert_eq!(data[7], true);
    }

    #[test]
    fn test_binary_array_iter(){
        let data = binnary_array::BinaryArray::new(100, true);
        for i in 0..data.len(){
            assert_eq!(data[i],true,"{}",i)
        }
    }

    #[bench]
    fn bench_binary_array_iter(b: &mut Bencher) {
        b.iter(|| {
            let binnary_array = BinaryArray::new(1_000_000, true);
            for _ in binnary_array {}
        });
    }

    #[bench]
    fn bench_sieve_run(b: &mut Bencher) {
        b.iter(|| {
            let mut sieve = SieveOfEratosthenes::new(1_000_000);
            sieve.run();
        });
    }
}
