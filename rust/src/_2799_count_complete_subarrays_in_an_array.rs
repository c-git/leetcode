//! Solution for https://leetcode.com/problems/count-complete-subarrays-in-an-array
//! 2799. Count Complete Subarrays in an Array

use std::collections::HashMap;

impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        // Values found in subarray
        let mut values_included = 0;
        // Frequency of values found in subarray (initialized to 0's over entire array)
        let mut values_freq: HashMap<i32, u16> = HashMap::new();

        // Find what the distinct values are
        for num in nums.iter().copied() {
            values_freq.entry(num).or_default();
        }

        let total_distinct_values = values_freq.keys().count();
        let mut left = 0;
        for (right, num) in nums.iter().enumerate() {
            let right_freq = values_freq
                .get_mut(num)
                .expect("all values should already be included");
            *right_freq += 1;
            if *right_freq == 1 {
                values_included += 1;
            }
            while values_included == total_distinct_values {
                result += nums.len() - right;
                let left_freq = values_freq.get_mut(&nums[left]).unwrap();
                *left_freq -= 1;
                if *left_freq == 0 {
                    values_included -= 1;
                }
                left += 1;
            }
        }
        result as _
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,3,1,2,2], 4)]
    #[case(vec![5,5,5,5], 10)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::count_complete_subarrays(nums);
        assert_eq!(actual, expected);
    }
}
