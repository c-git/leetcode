//! Solution for https://leetcode.com/problems/make-sum-divisible-by-p
//! 1590. Make Sum Divisible by P

use std::collections::HashMap;

impl Solution {
    /// Based on https://www.youtube.com/watch?v=7FJrMTpadRI
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let remainder_to_remove = nums.iter().fold(0, |acc, x| (acc + x) % p);
        if remainder_to_remove == 0 {
            return 0;
        }
        let mut result = nums.len() as i32;
        let mut curr_sum = 0;
        // Stores the last index where that remainder was seen
        let mut prev_sum = HashMap::new();
        prev_sum.insert(0, -1);

        for (i, num) in nums.iter().enumerate() {
            let i = i as i32;
            curr_sum = (curr_sum + num) % p;
            let prefix_to_remove = (p + curr_sum - remainder_to_remove) % p;
            if let Some(prev_idx) = prev_sum.get(&prefix_to_remove) {
                let length = i - prev_idx;
                result = result.min(length);
            }
            prev_sum.insert(curr_sum, i);
        }

        if result == nums.len() as i32 {
            -1
        } else {
            result
        }
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![3,1,4,2], 6, 1)]
    #[case(vec![6,3,5,2], 9, 2)]
    #[case(vec![1,2,3], 3, 0)]
    fn case(#[case] nums: Vec<i32>, #[case] p: i32, #[case] expected: i32) {
        let actual = Solution::min_subarray(nums, p);
        assert_eq!(actual, expected);
    }
}
