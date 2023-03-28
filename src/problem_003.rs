mod problem_003 {
    struct Euler;

    impl Euler {
        fn exec() -> u64 {
            // ~ 6μs
            let mut n: u64 = 600851475143;
            let mut last_factor: u64;
            if n % 2 == 0 {
                last_factor = 2;
                n = n / 2;

                while n % 2 == 0 {
                    n = n / 2;
                }
            } else {
                last_factor = 1;
            }
            let mut factor = 3;
            let mut max_factor = (n as f32).sqrt();
            while n > 1 && factor as f32 <= max_factor {
                if n % factor == 0 {
                    n = n / factor;
                    last_factor = factor;
                    while n % factor == 0 {
                        n = n / factor;
                    }
                    max_factor = (n as f32).sqrt();
                }
                factor = factor + 2;
            }
            if n == 1 {
                return last_factor;
            } else {
                return n;
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::problem_003::problem_003::Euler;
        use std::time::Instant;

        #[test]
        fn test_exec() {
            let start = Instant::now();
            let result = Euler::exec();
            let duration = start.elapsed();
            println!("Answer: {}, Took: {:?}", result, duration);
            assert_eq!(6857, result);
        }
    }
}
