//! Solution for https://leetcode.com/problems/count-of-interesting-subarrays
//! 2845. Count of Interesting Subarrays

use std::collections::HashMap;

impl Solution {
    /// Based on editorial
    pub fn count_interesting_subarrays(nums: Vec<i32>, modulo: i32, k: i32) -> i64 {
        let mut result = 0i64;

        let mut prefix_sum = 0;
        let mut previous_sum_freq: HashMap<i32, i32> = HashMap::new();
        previous_sum_freq.insert(0, 1);
        for num in nums {
            if num % modulo == k {
                prefix_sum += 1;
            }
            let prev_needed = (prefix_sum + modulo - k) % modulo;
            result += previous_sum_freq
                .get(&prev_needed)
                .copied()
                .unwrap_or_default() as i64;
            *previous_sum_freq.entry(prefix_sum % modulo).or_default() += 1;
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
    #[case(vec![3,2,4], 2, 1, 3)]
    #[case(vec![3,1,9,6], 3, 0, 2)]
    #[case(vec![1,2,1,2,1,2,1], 2, 1, 16)]
    fn case(#[case] nums: Vec<i32>, #[case] modulo: i32, #[case] k: i32, #[case] expected: i64) {
        let actual = Solution::count_interesting_subarrays(nums, modulo, k);
        assert_eq!(actual, expected);
    }
}
