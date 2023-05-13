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

    #[test]
    fn case1() {
        let low = 3;
        let high = 3;
        let zero = 1;
        let one = 1;
        let expected = 8;
        let actual = Solution::count_good_strings(low, high, zero, one);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case2() {
        let low = 2;
        let high = 3;
        let zero = 1;
        let one = 2;
        let expected = 5;
        let actual = Solution::count_good_strings(low, high, zero, one);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case3() {
        let low = 1;
        let high = 100_000;
        let zero = 1;
        let one = 1;
        let expected = 215447031;
        let actual = Solution::count_good_strings(low, high, zero, one);
        assert_eq!(actual, expected);
    }
}
