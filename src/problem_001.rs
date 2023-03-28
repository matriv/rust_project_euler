mod problem_001 {
    use std::collections::HashSet;

    pub struct Euler;

    impl Euler {
        pub fn exec() -> u32 {
            let mut sum: u32 = 0;

            // Naive approach ~ 130μs
            // for i in 1..1000 {
            //     let last_digit = i.to_string().chars().last().unwrap();
            //     if last_digit == '0' || last_digit == '5' {
            //         sum += i;
            //     } else if i % 3 == 0 {
            //         sum += i;
            //     }
            // }

            // Check for last digit for / 5 ~ 130μs
            // for i in 1..1000 {
            //     let last_digit = i.to_string().chars().last().unwrap();
            //     if last_digit == '0' || last_digit == '5' {
            //         sum += i;
            //     } else if i % 3 == 0 {
            //         sum += i;
            //     }
            // }

            // Iterations 100μs
            let mut set = HashSet::new();
            let mut i = 3;
            while i < 1000 {
                sum += i;
                if i % 5 == 0 {
                    set.insert(i);
                }
                i += 3;
            }
            let mut i = 5;
            while i < 1000 {
                if !set.contains(&i) {
                    sum += i;
                }
                i += 5;
            }

            // Series of 3 + 6 + 9 + ...
            //         + 5 + 10 + 15 + ...
            //         - 15 - 30 - 45 ... (multiples of 15 which are divided both by 3 & 5)
            // ~ 60ns
            // sum += ((1 + 333) * 3 * 333) / 2;
            // sum += ((1 + 199) * 5 * 199) / 2;
            // sum -= ((1 + 66) * 15 * 66) / 2;

            return sum;
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
