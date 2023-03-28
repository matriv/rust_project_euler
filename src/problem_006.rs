mod problem_006 {
    pub struct Euler;
    impl Euler {
        pub fn exec() -> u32 {
            // ~ 7μs
            // let mut sum_square: i64 = 0;
            // let mut square_sum: i64 = 0;
            // for i in 1..101 {
            //     sum_square += i * i;
            //     square_sum += i;
            // }
            // return (square_sum * square_sum) - sum_square;

            // ~ 110ns
            let limit = 100;
            let sum = limit * (limit + 1) / 2;
            let sum_sq = (2 * limit + 1) * (limit + 1) * limit / 6;
            return (sum * sum) - sum_sq;
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::problem_006::problem_006::Euler;
        use std::time::Instant;

        #[test]
        fn test_exec() {
            let start = Instant::now();
            let result = Euler::exec();
            let duration = start.elapsed();
            println!("Answer: {}, Took: {:?}", result, duration);
            assert_eq!(25164150, result);
        }
    }
}
