#[cfg(test)]
mod tests {
    extern crate test;

    use crate::{
        binnary_array::{self, BinaryArray},
        settings::get_settings,
        sieves::SieveOfEratosthenes,
    };
    use test::Bencher;

    #[test]
    fn test_get_primes() {
        get_settings().show_bar = false;
        let mut sieve = SieveOfEratosthenes::new(10);

        assert_eq!(sieve.get_primes(), vec![2, 3, 5, 7]);
    }

    #[test]
    fn test_get_limit() {
        get_settings().show_bar = false;
        let sieve = SieveOfEratosthenes::new(100);

        assert_eq!(sieve.get_limit(), 100);
    }

    #[test]
    fn test_set_limit() {
        let mut sieve = SieveOfEratosthenes::new(100);

        sieve.set_limit(200);
        assert_eq!(sieve.get_limit(), 200);
    }

    #[test]
    fn test_get_primes_after_changing_limit() {
        get_settings().show_bar = false;
        let mut sieve = SieveOfEratosthenes::new(100);

        sieve.set_limit(20);
        assert_eq!(sieve.get_primes(), vec![2, 3, 5, 7, 11, 13, 17, 19]);
    }

    #[test]
    fn test_binary_array_read() {
        let data = binnary_array::BinaryArray::new(8, false);
        assert_eq!(data[7], false);
    }

    #[test]
    fn test_binary_array_write() {
        let mut data = binnary_array::BinaryArray::new(10, false);
        data[7] = true;
        assert_eq!(data[7], true);
    }

    #[bench]
    fn bench_binary_array_iter(b: &mut Bencher) {
        b.iter(|| {
            let binnary_array = BinaryArray::new(1_000_000, true);
            for _ in binnary_array {
            }
        });
    }

    #[bench]
    fn bench_sieve_run(b: &mut Bencher){
        b.iter(|| {
            let mut sieve =SieveOfEratosthenes::new(1_000_000);
            sieve.run();
        });
    }
}
