mod problem_009 {
    pub struct Euler;
    impl Euler {
        // ~ 1ms
        pub fn exec() -> u32 {
            let s = 1000; // or whatever the perimeter should be
            for a in 3..(s - 3) / 2 {
                for b in (a + 1)..(s - 1 - a) / 2 {
                    let c = s - a - b;
                    if c * c == a * a + b * b {
                        return a * b * c;
                    }
                }
            }
            return 0;
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::problem_009::problem_009::Euler;
        use std::time::Instant;

        #[test]
        fn test_exec() {
            let start = Instant::now();
            let result = Euler::exec();
            let duration = start.elapsed();
            println!("Answer: {}, Took: {:?}", result, duration);
            assert_eq!(31875000, result);
        }
    }
}
