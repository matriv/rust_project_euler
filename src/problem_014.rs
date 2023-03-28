mod problem_014 {
    pub struct Euler;
    impl Euler {
        // ~ 250ms
        pub fn exec() -> u64 {
            let mut chain;
            let mut max_chain = 0;
            let mut result = 1;
            for i in 1..1_000_000 {
                chain = 0;
                let mut n = i;
                loop {
                    n = Self::f(n);
                    chain += 1;
                    if n == 1 {
                        break;
                    }
                }
                if chain > max_chain {
                    max_chain = chain;
                    result = i;
                }
            }
            return result;
        }

        fn f(n: u64) -> u64 {
            if n % 2 == 0 {
                return n / 2;
            } else {
                return 3 * n + 1;
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::problem_014::problem_014::Euler;
        use std::time::Instant;

        #[test]
        fn test_exec() {
            let start = Instant::now();
            let result = Euler::exec();
            let duration = start.elapsed();
            println!("Answer: {}, Took: {:?}", result, duration);
            assert_eq!(837799, result);
        }
    }
}
