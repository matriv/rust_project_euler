mod problem_001 {
    pub struct Euler;

    impl Euler {
        pub fn exec() -> u32 {
            // Naive approach ~ 20μs
            return (1..1000).filter(|&n| n % 3 == 0 || n % 5 == 0).sum();

            // Series of 3 + 6 + 9 + ...
            //         + 5 + 10 + 15 + ...
            //         - 15 - 30 - 45 ... (multiples of 15 which are divided both by 3 & 5)
            // ~ 60ns
            // let mut sum: u32 = 0;
            // sum += ((1 + 333) * 3 * 333) / 2;
            // sum += ((1 + 199) * 5 * 199) / 2;
            // sum -= ((1 + 66) * 15 * 66) / 2;
            // return sum;
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::problem_001::problem_001::Euler;
        use std::time::Instant;

        #[test]
        fn test_exec() {
            let start = Instant::now();
            let result = Euler::exec();
            let duration = start.elapsed();
            println!("Answer: {}, Took: {:?}", result, duration);
            assert_eq!(233168, result);
        }
    }
}
