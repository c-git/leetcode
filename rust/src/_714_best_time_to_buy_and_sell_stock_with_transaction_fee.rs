//! Solution for https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-transaction-fee
//! 714. Best Time to Buy and Sell Stock with Transaction Fee

use std::cmp::max;

impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        // Based on editorial
        let n = prices.len();
        let mut hold = vec![0; n];
        let mut free = vec![0; n];

        // In order to hold a stock on day 0, we have no other choice but to buy it for prices[0].
        hold[0] = -prices[0];

        for (i, price) in prices.iter().enumerate().skip(1) {
            hold[i] = max(hold[i - 1], free[i - 1] - price);
            free[i] = max(free[i - 1], hold[i - 1] + price - fee);
        }

        free[n - 1]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,3,2,8,4,9], 2, 8)]
    #[case(vec![1,3,7,5,10,3], 3, 6)]
    #[case(vec![1,3,7,5,10,3], 15, 0)]
    fn case(#[case] prices: Vec<i32>, #[case] fee: i32, #[case] expected: i32) {
        let actual = Solution::max_profit(prices, fee);
        assert_eq!(actual, expected);
    }
}
