mod problem_005 {
    use round::round_down;
    pub struct Euler;
    impl Euler {
        pub fn exec() -> u32 {
            // ~ 7μs
            let p = [1, 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31];
            let k: u32 = 20;
            let mut a = [0; 20];
            let mut n: u32 = 1;
            let mut i = 1;
            let mut check = true;
            let limit = (k as f32).sqrt();
            while p[i] <= k {
                a[i] = 1;
                if check {
                    if p[i] as f32 <= limit {
                        a[i] = round_down(((k as f32).ln() / (p[i] as f32).ln()).into(), 0) as u32;
                    } else {
                        check = false;
                    }
                }
                n *= p[i].pow(a[i] as u32);
                i += 1;
            }
            return n;
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::problem_005::problem_005::Euler;
        use std::time::Instant;

        #[test]
        fn test_exec() {
            let start = Instant::now();
            let result = Euler::exec();
            let duration = start.elapsed();
            println!("Answer: {}, Took: {:?}", result, duration);
            assert_eq!(232792560, result);
        }
    }
}
