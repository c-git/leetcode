//! Solution for https://leetcode.com/problems/coin-change-ii
//! 518. Coin Change II

impl Solution {
    pub fn change(amount: i32, mut coins: Vec<i32>) -> i32 {
        coins.sort_unstable();
        Self::change_(amount, &coins)
    }
    pub fn change_(amount: i32, coins: &[i32]) -> i32 {
        if amount == 0 {
            return 1;
        }

        if coins.is_empty() {
            return 0;
        }

        let mut result = 0;

        let largest_val = coins.last().expect("Checked for empty above");
        let max_supported = amount / largest_val;
        let other_coins = &coins[..coins.len() - 1];
        for i in 1..=max_supported {
            result += Self::change_(amount - largest_val * i, other_coins)
        }

        result + Self::change_(amount, other_coins)
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
    fn case(#[case] amount: i32, #[case] coins: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::change(amount, coins);
        assert_eq!(actual, expected);
    }
}
