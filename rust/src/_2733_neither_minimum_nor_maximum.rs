//! Solution for https://leetcode.com/problems/neither-minimum-nor-maximum
//! 2733. Neither Minimum nor Maximum

use std::collections::BTreeSet;

impl Solution {
    pub fn find_non_min_or_max(nums: Vec<i32>) -> i32 {
        Self::find_non_min_or_max_(&nums).unwrap_or(-1)
    }

    pub fn find_non_min_or_max_(nums: &[i32]) -> Option<i32> {
        let mut set = BTreeSet::new();
        for &num in nums {
            set.insert(num);
            debug_assert!(set.len() <= 3);
            if set.len() >= 3 {
                return Some(*set.iter().nth(1).unwrap());
            }
        }
        None // All numbers were either the min or the max
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![3,2,1,4], 2)]
    #[case(vec![1,2], -1)]
    #[case(vec![2,1,3], 2)]
    #[case(vec![1,2,2,1,1], -1)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::find_non_min_or_max(nums);
        assert_eq!(actual, expected);
    }
}
