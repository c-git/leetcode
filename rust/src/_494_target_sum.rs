//! Solution for https://leetcode.com/problems/target-sum
//! 494. Target Sum

use std::collections::HashMap;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let mut memo = HashMap::new();
        Self::find_target_sum_ways_(&nums[..], target, 0, &mut memo)
    }

    fn find_target_sum_ways_(
        nums: &[i32],
        target: i32,
        partial: i32,
        memo: &mut HashMap<(usize, i32), i32>,
    ) -> i32 {
        let key = (nums.len(), partial);
        if let Some(&x) = memo.get(&key) {
            return x;
        }
        if nums.is_empty() {
            return if partial == target { 1 } else { 0 };
        }
        let mut result = 0;
        result += Self::find_target_sum_ways_(&nums[1..], target, partial + nums[0], memo);
        result += Self::find_target_sum_ways_(&nums[1..], target, partial - nums[0], memo);
        memo.insert(key, result);
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
    #[case(vec![1,1,1,1,1], 3, 5)]
    #[case(vec![1], 1, 1)]
    fn case(#[case] nums: Vec<i32>, #[case] target: i32, #[case] expected: i32) {
        let actual = Solution::find_target_sum_ways(nums, target);
        assert_eq!(actual, expected);
    }
}
