//! Solution for https://leetcode.com/problems/coin-change
//! 322. Coin Change

impl Solution {
    // Based on https://www.youtube.com/watch?v=YzIjgBONTJk
    pub fn coin_change(mut coins: Vec<i32>, amount: i32) -> i32 {
        coins.sort_unstable();
        let coins: Vec<_> = coins.into_iter().map(|x| x as usize).collect(); // Converted to simplify code
        let amount = amount as usize;
        let mut dp = vec![usize::MAX; amount + 1];
        dp[0] = 0;
        for sub_amount in 1..=amount {
            for coin in coins.iter().copied() {
                if coin > sub_amount {
                    break; // No more coins can be used
                }
                dp[sub_amount] = dp[sub_amount].min(dp[sub_amount - coin].saturating_add(1));
            }
        }
        if dp[amount] == usize::MAX {
            -1
        } else {
            dp[amount] as i32
        }
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,5], 11, 3)]
    #[case(vec![2], 3, -1)]
    #[case(vec![1], 0, 0)]
    fn case(#[case] coins: Vec<i32>, #[case] amount: i32, #[case] expected: i32) {
        let actual = Solution::coin_change(coins, amount);
        assert_eq!(actual, expected);
    }
}
