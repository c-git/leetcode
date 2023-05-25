impl Solution {
    pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        // Converted version of editorial code

        let n = n as usize;
        let k = k as usize;
        let max_pts = max_pts as usize;
        let mut dp = vec![0.0; n + 1];
        dp[0] = 1.0;
        let mut sum = if k > 0 { 1.0 } else { 0.0 };
        for i in 1..n + 1 {
            dp[i] = sum / max_pts as f64;
            if i < k {
                sum += dp[i]
            }
            if i >= max_pts && i - max_pts < k {
                sum -= dp[i - max_pts]
            }
        }
        dp[k..].iter().sum()
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(10, 1, 10, 1.0)]
    #[case(6, 1, 10, 0.6)]
    #[case(21, 17, 10, 0.73278)]
    fn case(#[case] n: i32, #[case] k: i32, #[case] max_pts: i32, #[case] expected: f64) {
        let actual = Solution::new21_game(n, k, max_pts);
        assert_eq!(format!("{actual:.5}"), format!("{expected:.5}"));
    }
}
