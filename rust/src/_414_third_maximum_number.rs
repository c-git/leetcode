//! Solution for https://leetcode.com/problems/third-maximum-number
//! 414. Third Maximum Number

use std::collections::BTreeSet;

impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let k = 3;

        let mut set = BTreeSet::new();

        for &num in nums.iter() {
            set.insert(num);
            if set.len() > k {
                set.pop_first();
            }
        }

        match set.len() {
            0 => unreachable!("Constraint guarantees there is at least 1 value"),
            1..=2 => set.pop_last().unwrap(), // No third return the last
            3 => set.pop_first().unwrap(),
            _ => unreachable!("Loop should keep value at max 3"),
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
    #[case(vec![3,2,1], 1)]
    #[case(vec![1,2], 2)]
    #[case(vec![2,2,3,1], 1)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::third_max(nums);
        assert_eq!(actual, expected);
    }
}
