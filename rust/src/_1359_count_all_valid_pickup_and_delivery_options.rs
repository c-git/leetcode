//! Solution for https://leetcode.com/problems/count-all-valid-pickup-and-delivery-options
//! 1359. Count All Valid Pickup and Delivery Options

impl Solution {
    // Source: https://leetcode.com/problems/count-all-valid-pickup-and-delivery-options/solutions/4024280/99-57-dp-math-recursion/?envType=daily-question&envId=2023-09-10
    // Video version: https://www.youtube.com/watch?v=wj3vBARk8-U
    const MOD: i64 = 1_000_000_007;

    pub fn count_orders(n: i32) -> i32 {
        let mut count: i64 = 1;
        for i in 2..=n as i64 {
            count = (count * (2 * i - 1) * i) % Self::MOD;
        }
        count as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(1, 1)]
    #[case(2, 6)]
    #[case(3, 90)]
    fn case(#[case] n: i32, #[case] expected: i32) {
        let actual = Solution::count_orders(n);
        assert_eq!(actual, expected);
    }
}
