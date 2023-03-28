mod problem_007 {
    pub struct Euler;
    impl Euler {
        pub fn exec() -> u32 {
            // ~ 2ms
            let limit = 10000;
            let mut count = 1; //we know that 2 is prime
            let mut candidate = 1;
            while count <= limit {
                candidate = candidate + 2;
                if Self::is_prime(candidate) {
                    count += 1;
                }
            }
            return candidate;
        }

        fn is_prime(n: u32) -> bool {
            use round::round_down;
            if n == 1 {
                return false;
            } else if n < 4 {
                // 2 and 3 are prime
                return true;
            } else if n % 2 == 0 {
                return false;
            } else if n < 9 {
                // we have already excluded 4,6 and 8.
                return true;
            } else if n % 3 == 0 {
                return false;
            } else {
                let r = round_down((n as f64).sqrt(), 0); // n rounded to the greatest integer r so that r*r<=n
                let mut f = 5;
                while f as f64 <= r {
                    if n % f == 0 {
                        return false;
                    }
                    if n % (f + 2) == 0 {
                        return false;
                    }
                    f = f + 6;
                }
                return true;
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::problem_007::problem_007::Euler;
        use std::time::Instant;

        #[test]
        fn test_exec() {
            let start = Instant::now();
            let result = Euler::exec();
            let duration = start.elapsed();
            println!("Answer: {}, Took: {:?}", result, duration);
            assert_eq!(104743, result);
        }
    }
}
