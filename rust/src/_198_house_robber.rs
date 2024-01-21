//! Solution for https://leetcode.com/problems/house-robber
//! 198. House Robber

use std::mem;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return nums[0];
        }
        debug_assert!(
            n >= 2,
            "Checked above that it's not 1 and constraint says it's 1 or greater"
        );

        let mut back_two = nums[0];
        let mut back_one = nums[1];

        for current in nums.iter().skip(2) {
            back_two += current;
            mem::swap(&mut back_one, &mut back_two);
        }
        back_one.max(back_two)
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,3,1],  4)]
    #[case(vec![2,7,9,3,1], 12)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::rob(nums);
        assert_eq!(actual, expected);
    }
}
