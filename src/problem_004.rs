mod problem_004 {
    pub struct Euler;
    impl Euler {
        pub fn exec() -> u32 {
            // 1ms
            let mut result: u32 = 0;
            for x in (100..1000).rev() {
                for y in (100..x).rev() {
                    let temp_result = x * y;
                    if temp_result <= result {
                        break;
                    }
                    let result_as_str = temp_result.to_string();
                    let mut is_palindrome = true;
                    for i in 0..result_as_str.len() - 1 / 2 {
                        if result_as_str.as_bytes()[i]
                            != result_as_str.as_bytes()[result_as_str.len() - i - 1]
                        {
                            is_palindrome = false;
                            break;
                        }
                    }
                    if is_palindrome {
                        if temp_result > result {
                            result = temp_result;
                        }
                    }
                }
            }
            return result;
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::problem_004::problem_004::Euler;
        use std::time::Instant;

        #[test]
        fn test_exec() {
            let start = Instant::now();
            let result = Euler::exec();
            let duration = start.elapsed();
            println!("Answer: {}, Took: {:?}", result, duration);
            assert_eq!(906609, result);
        }
    }
}
