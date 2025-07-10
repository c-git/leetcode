//! Solution for https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii
//! 122. Best Time to Buy and Sell Stock II

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut curr_price = prices[0];
        for price in prices {
            if curr_price < price {
                result += price - curr_price;
            }
            curr_price = price;
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
    #[case(vec![7,1,5,3,6,4], 7)]
    #[case(vec![1,2,3,4,5], 4)]
    #[case(vec![7,6,4,3,1], 0)]
    fn case(#[case] prices: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::max_profit(prices);
        assert_eq!(actual, expected);
    }
}
