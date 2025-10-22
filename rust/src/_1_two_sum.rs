//! Solution for https://leetcode.com/problems/two-sum
//! 1. Two Sum

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut missing_diff: HashMap<i32, i32> = HashMap::new();
        for (i, num) in nums.into_iter().enumerate() {
            if let Some(other_idx) = missing_diff.get(&(target - num)) {
                return vec![*other_idx, i as i32];
            }
            missing_diff.insert(num, i as i32);
        }
        unreachable!("solution existence is guaranteed by LC")
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![2,7,11,15], 9, vec![0,1])]
    #[case(vec![3,2,4], 6, vec![1,2])]
    #[case(vec![3,3], 6, vec![0,1])]
    fn case(#[case] nums: Vec<i32>, #[case] target: i32, #[case] expected: Vec<i32>) {
        let actual = Solution::two_sum(nums, target);
        assert_eq!(actual, expected);
    }
}
