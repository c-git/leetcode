//! Solution for https://leetcode.com/problems/count-the-number-of-good-subarrays
//! 2537. Count the Number of Good Subarrays

use std::collections::HashMap;

impl Solution {
    pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
        let mut result = 0;
        let mut freq_tracker: HashMap<i32, i32> = HashMap::new();
        let mut left = 0;
        let mut pair_count = 0;
        for (right, right_val) in nums.iter().copied().enumerate() {
            let right_entry = freq_tracker.entry(right_val).or_default();
            pair_count += *right_entry;
            *right_entry += 1;
            while pair_count >= k {
                // counting the remaining positions taken from editorial
                result += (nums.len() - right) as i64;
                let left_entry = freq_tracker
                    .get_mut(&nums[left])
                    .expect("should have already been added");
                *left_entry -= 1;
                pair_count -= *left_entry;
                left += 1;
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
    #[case(vec![1,1,1,1,1], 10, 1)]
    #[case(vec![3,1,4,3,2,2,4], 2, 4)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i64) {
        let actual = Solution::count_good(nums, k);
        assert_eq!(actual, expected);
    }
}
