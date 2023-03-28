mod problem_002 {
    pub struct Euler;

    impl Euler {
        pub fn exec() -> u32 {
            // 200ns
            let mut prev1 = 1;
            let mut prev2 = 1;

            let mut sum = 0;
            loop {
                let fib = prev1 + prev2;
                prev1 = prev2;
                prev2 = fib;
                if fib >= 4_000_000 {
                    break;
                }
                if fib % 2 == 0 {
                    sum += fib;
                }
            }
            return sum;
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::problem_002::problem_002::Euler;
        use std::time::Instant;

        #[test]
        fn test_exec() {
            let start = Instant::now();
            let result = Euler::exec();
            let duration = start.elapsed();
            println!("Answer: {}, Took: {:?}", result, duration);
            assert_eq!(4613732, result);
        }
    }
}
