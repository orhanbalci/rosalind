mod math_helper {
    fn combinations(n: u32, k: u32) -> u32 {
        assert!(n >= k);
        let upper_part = (n - k + 1..n + 1).fold(1_u32, |mut accu, x| {
            accu = accu * x;
            accu
        });
        upper_part / factorial(k)
    }

    fn factorial(n: u32) -> u32 {
        (0..n + 1).fold(1_u32, |mut accu, x| {
            match x {
                0 => accu = 1,
                _ => accu = accu * x,
            }
            accu
        })
    }

    mod test {
        #[test]
        fn test_factorial() {
            assert_eq!(super::factorial(0), 1_u32);
            assert_eq!(super::factorial(1), 1_u32);
            assert_eq!(super::factorial(5), 120_u32);
        }

        #[test]
        fn test_combination() {
            assert_eq!(super::combinations(5, 2), 10_u32);
        }
    }
}
