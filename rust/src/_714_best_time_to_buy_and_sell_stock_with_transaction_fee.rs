//! Solution for https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-transaction-fee
//! 714. Best Time to Buy and Sell Stock with Transaction Fee

use std::cmp::max;

impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        // Based on editorial
        let mut last_hold = -prices[0]; // In order to hold a stock on day 0, we have no other choice but to buy it for prices[0].
        let mut last_free = 0;

        for price in prices.iter().skip(1) {
            let new_hold = max(last_hold, last_free - price);
            last_free = max(last_free, last_hold + price - fee);
            last_hold = new_hold;
        }

        last_free
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
