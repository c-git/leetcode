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
                Self::pop_first(&mut set);
            }
        }

        match set.len() {
            0 => unreachable!("Constraint guarantees there is at least 1 value"),
            1..=2 => Self::pop_last(&mut set), // No third return the last (largest value)
            3 => Self::pop_first(&mut set),    // The only three left the smallest is the third
            _ => unreachable!("Loop should keep value at max 3"),
        }
    }

    fn pop_first(set: &mut BTreeSet<i32>) -> i32 {
        let first = *set
            .iter()
            .next()
            .expect("Only expected to be called if the set is non-empty");
        set.remove(&first);
        first
    }

    fn pop_last(set: &mut BTreeSet<i32>) -> i32 {
        let last = *set
            .iter()
            .next_back()
            .expect("Only expected to be called if the set is non-empty");
        set.remove(&last);
        last
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
