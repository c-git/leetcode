//! Solution for https://leetcode.com/problems/best-time-to-buy-and-sell-stock
//! 121. Best Time to Buy and Sell Stock

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut low = i32::MAX;
        for price in prices {
            if price < low {
                low = price;
            } else {
                let diff = price - low;
                result = result.max(diff);
            }
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
    #[case(vec![7,1,5,3,6,4], 5)]
    #[case(vec![7,6,4,3,1], 0)]
    fn case(#[case] prices: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::max_profit(prices);
        assert_eq!(actual, expected);
    }
}
