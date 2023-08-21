//! Solution for https://leetcode.com/problems/powx-n
//! 50. Pow(x, n)

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }
        if n < 0 {
            return 1.0 / Self::my_pow(x, -n);
        }
        let mut result = 1.0;
        for _ in 0..n {
            result *= x;
        }
        result
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(2.00000, 10, 1024.0)]
    #[case(2.10000, 3, 9.261)]
    #[case(2.00000, -2, 0.25)]
    fn case(#[case] x: f64, #[case] n: i32, #[case] expected: f64) {
        let actual = Solution::my_pow(x, n);
        assert!((actual - expected).abs() < 1e-5, "Assertion failed: actual {actual:.5} but expected {expected:.5}. Diff is more than 1e-5.");
    }
}
