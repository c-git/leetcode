//! Solution for https://leetcode.com/problems/coin-change-ii
//! 518. Coin Change II

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        // Solution from editorial
        let amount = amount as usize;
        let coins: Vec<usize> = coins.into_iter().map(|x| x as usize).collect();
        let n = coins.len();

        // Each row is indexed by the starting index of the coins you can use, each column is indexed by a target amount
        let mut dp = vec![vec![0; amount + 1]; n + 1];
        for row in dp.iter_mut() {
            row[0] = 1;
        }

        for start_coin_idx in (0..n).rev() {
            for target_val in 1..=amount {
                if coins[start_coin_idx] > target_val {
                    dp[start_coin_idx][target_val] = dp[start_coin_idx + 1][target_val]
                } else {
                    dp[start_coin_idx][target_val] = dp[start_coin_idx + 1][target_val]
                        + dp[start_coin_idx][target_val - coins[start_coin_idx]]
                }
            }
        }
        dp[0][amount]
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
