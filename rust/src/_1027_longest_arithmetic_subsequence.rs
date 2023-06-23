//! Solution for https://leetcode.com/problems/longest-arithmetic-subsequence
//! 1027. Longest Arithmetic Subsequence

use std::collections::HashMap;

impl Solution {
    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        // After reading editorial
        let mut dp = HashMap::new();

        for right in 1..nums.len() {
            for left in 0..right {
                let diff = nums[right] - nums[left];
                dp.insert((right, diff), dp.get(&(left, diff)).unwrap_or(&1) + 1);
            }
        }

        *dp.values()
            .max()
            .expect("By constraint there are at least 2 items in input")
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![3,6,9,12], 4)]
    #[case(vec![9,4,7,2,10], 3)]
    #[case(vec![20,1,15,3,10,5,8], 4)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::longest_arith_seq_length(nums);
        assert_eq!(actual, expected);
    }
}
