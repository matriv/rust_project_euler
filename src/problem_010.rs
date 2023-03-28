mod problem_010 {
    use primes::{PrimeSet, Sieve};

    pub struct Euler;
    impl Euler {
        // ~ 400ms
        pub fn exec() -> u64 {
            let mut pset = Sieve::new();
            let mut sum: u64 = 0;
            for (_i, p) in pset.iter().enumerate() {
                if p > 2_000_000 {
                    break;
                }
                sum += p;
            }
            return sum;
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::problem_010::problem_010::Euler;
        use std::time::Instant;

        #[test]
        fn test_exec() {
            let start = Instant::now();
            let result = Euler::exec();
            let duration = start.elapsed();
            println!("Answer: {}, Took: {:?}", result, duration);
            assert_eq!(142913828922, result);
        }
    }
}
