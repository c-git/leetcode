impl Solution {
    pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
        // After reading editorial
        debug_assert!(low >= 0);
        debug_assert!(low <= high);
        let high = high as usize;
        let low = low as usize;
        let zero = zero as usize;
        let one = one as usize;
        let result_mod = 1_000_000_007;
        let mut dp = vec![0; high + 1];
        dp[0] = 1;

        for i in 1..=high {
            if i >= zero {
                dp[i] = dp[i - zero];
            }
            if i >= one {
                dp[i] = (dp[i] + dp[i - one]) % result_mod;
            }
        }

        dp.iter()
            .skip(low)
            .take(high - low + 1)
            .fold(0, |acc, x| (acc + x) % result_mod)
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(3, 3, 1, 1, 8)]
    #[case(2, 3, 1, 2, 5)]
    #[case(1, 100_000, 1, 1, 215447031)]
    fn case(
        #[case] low: i32,
        #[case] high: i32,
        #[case] zero: i32,
        #[case] one: i32,
        #[case] expected: i32,
    ) {
        let actual = Solution::count_good_strings(low, high, zero, one);
        assert_eq!(actual, expected);
    }
}
