mod problem_012 {
    pub struct Euler;
    impl Euler {
        // ~ 24μs
        pub fn exec() -> u32 {
            let mut result: u32 = 0;
            let mut cnt: u32;
            let mut sqrt: u32;
            for i in 1..u32::MAX {
                result += i;
                cnt = 1;
                sqrt = (result as f32).sqrt() as u32 + 1;
                for j in 1..sqrt {
                    if result % j == 0 {
                        cnt += 2;
                    }
                    if cnt == 501 {
                        return result;
                    }
                }
            }
            return result;
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::problem_012::problem_012::Euler;
        use std::time::Instant;

        #[test]
        fn test_exec() {
            let start = Instant::now();
            let result = Euler::exec();
            let duration = start.elapsed();
            println!("Answer: {}, Took: {:?}", result, duration);
            assert_eq!(76576500, result);
        }
    }
}
