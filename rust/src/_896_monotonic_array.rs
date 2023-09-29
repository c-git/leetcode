//! Solution for https://leetcode.com/problems/monotonic-array
//! 896. Monotonic Array

use std::cmp::Ordering;

impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        if nums.len() <= 2 {
            return true;
        }

        let mut is_increasing: Option<bool> = None;
        for i in 1..nums.len() {
            let cmp = nums[i - 1].cmp(&nums[i]);
            match (is_increasing, cmp) {
                (None, Ordering::Less) => is_increasing = Some(false),
                (None, Ordering::Greater) => is_increasing = Some(true),
                (Some(expected_increase), Ordering::Less) if expected_increase => return false,
                (Some(expected_increase), Ordering::Greater) if !expected_increase => return false,
                _ => (), // All other cases need no action
            }
        }

        true
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,2,3], true)]
    #[case(vec![6,5,4,4], true)]
    #[case(vec![1,3,2], false)]
    #[case(vec![1,1,1,1,1], true)]
    #[case(vec![1,1,1,1,2], true)]
    #[case(vec![4,4,4,4,3], true)]
    #[case(vec![4,4,4,4,3,5], false)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: bool) {
        let actual = Solution::is_monotonic(nums);
        assert_eq!(actual, expected);
    }
}
