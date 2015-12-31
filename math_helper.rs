
pub fn combinations(n: u32, k: u32) -> f32 {
    assert!(n >= k);
    if n == k {
        return 1f32
    }

    let it = (1..k+1).rev();
    let combinations = (n - k + 1..n + 1).rev().zip(it).fold(1_f32, |mut accu, x| {
        accu *= x.0 as f32 ;
        accu /= x.1 as f32;
        accu
    });
    combinations
}

pub fn factorial(n: u32) -> f32 {
    (0..n + 1).fold(1_f32, |mut accu, x| {
        match x {
            0 => accu = 1_f32,
            _ => accu = accu * x as f32,
        }
        accu
    })
}

mod test {
    #[test]
    fn test_factorial() {
        assert_eq!(super::factorial(0), 1_f32);
        assert_eq!(super::factorial(1), 1_f32);
        assert_eq!(super::factorial(5), 120_f32);
    }    

    #[test]
    fn test_combination() {
        assert_eq!(super::combinations(5, 2), 10_f32);
        assert_eq!(super::combinations(6, 4), 15_f32);
    }
}
