#[cfg(test)]
mod tests{
    use crate::sieves::SieveOfEratosthenes;

    #[test]
    fn test_get_primes(){
        let mut sieve = SieveOfEratosthenes::new(10);

       assert_eq!(sieve.get_primes(),vec![2,3,5,7]);
    }

    #[test]
    fn test_get_limit(){
        let sieve = SieveOfEratosthenes::new(100);

        assert_eq!(sieve.get_limit(),100);
    }

    #[test]
    fn test_set_limit(){
        let mut sieve = SieveOfEratosthenes::new(100);

        sieve.set_limit(200);
        assert_eq!(sieve.get_limit(),200);
    }

    #[test]
    fn test_get_primes_after_changing_limit(){
        let mut sieve = SieveOfEratosthenes::new(100);

        sieve.set_limit(20);
        assert_eq!(sieve.get_primes(),vec![2,3,5,7,11,13,17,19]);
    }
}