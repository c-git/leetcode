//! Solution for https://leetcode.com/problems/maximum-subarray
//! 53. Maximum Subarray

use std::cmp::max;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        // Source: https://leetcode.com/problems/maximum-subarray/solutions/3315652/rust-dynamic-programming-kadane-s-algorithm-concise-solution/
        nums.iter()
            .fold((i32::MIN, 0), |(max_sum, current_sum), &num| {
                let new_sum = max(current_sum + num, num);
                (max(max_sum, new_sum), new_sum)
            })
            .0
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![-2,1,-3,4,-1,2,1,-5,4], 6)]
    #[case(vec![1], 1)]
    #[case(vec![5,4,-1,7,8], 23)]
    #[case(vec![-5,-4,-1,-7,8], 8)]
    #[case(vec![5,-4,-1,-7,-8], 5)]
    #[case(vec![-5,-4,1,-7,-8], 1)]
    #[case(vec![-5,-4,-1,-7,-8], -1)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::max_sub_array(nums);
        assert_eq!(actual, expected);
    }
}
