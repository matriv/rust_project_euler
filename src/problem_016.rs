mod problem_016 {
    use num_bigint::BigInt;
    use num_traits::FromPrimitive;

    pub struct Euler;
    impl Euler {
        // ~ 350μs
        fn exec(base: u32, exp: u32) -> u32 {
            let base: BigInt = FromPrimitive::from_u32(base).unwrap();
            num_traits::pow(base, exp as usize)
                .to_string()
                .chars()
                .filter_map(|c| c.to_digit(10))
                .sum()
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::problem_016::problem_016::Euler;
        use std::time::Instant;

        #[test]
        fn test_exec() {
            let start = Instant::now();
            let result = Euler::exec(20, 1000);
            let duration = start.elapsed();
            println!("Answer: {}, Took: {:?}", result, duration);
            assert_eq!(1366, result);
        }
    }
}
