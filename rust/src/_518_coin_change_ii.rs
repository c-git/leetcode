//! Solution for https://leetcode.com/problems/coin-change-ii
//! 518. Coin Change II

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        // Solution from editorial (See older commits which progressively get to this solution (easier to understand))
        let amount = amount as usize;
        let n = coins.len();

        // Each index stores the best we can do using the current prefix for a given target.
        let mut dp = vec![0; amount + 1];
        dp[0] = 1;

        for start_coin_idx in (0..n).rev() {
            for target_val in coins[start_coin_idx] as usize..=amount {
                dp[target_val] += dp[target_val - coins[start_coin_idx] as usize];
            }
        }
        dp[amount]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(5, vec![1,2,5], 4)]
    #[case(3, vec![2], 0)]
    #[case(10, vec![10], 1)]
    #[case(500, vec![3,5,7,8,9,10,11], 35_502_874)]
    fn case(#[case] amount: i32, #[case] coins: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::change(amount, coins);
        assert_eq!(actual, expected);
    }
}
