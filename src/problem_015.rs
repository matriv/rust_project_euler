mod problem_015 {
    pub struct Euler;
    impl Euler {
        // ~ 1μs
        pub fn exec(n: u64) -> u64 {
            let mut result: u64 = 1;
            for i in 1..n + 1 {
                result = result * (n + i) / i;
            }
            return result;
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::problem_015::problem_015::Euler;
        use std::time::Instant;

        #[test]
        fn test_exec() {
            let start = Instant::now();
            let result = Euler::exec(20);
            let duration = start.elapsed();
            println!("Answer: {}, Took: {:?}", result, duration);
            assert_eq!(137846528820, result);
        }
    }
}
