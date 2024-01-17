//! Solution for https://leetcode.com/problems/unique-number-of-occurrences
//! 1207. Unique Number of Occurrences

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut freq: HashMap<i32, usize> = HashMap::new();
        arr.iter().for_each(|x| *freq.entry(*x).or_default() += 1);
        let mut seen = HashSet::new();
        for val in freq.values() {
            if seen.contains(val) {
                return false;
            }
            seen.insert(val);
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
    #[case(vec![1,2,2,1,1,3], true)]
    #[case(vec![1,2], false)]
    #[case(vec![-3,0,1,-3,1,1,1,-3,10,0], true)]
    fn case(#[case] arr: Vec<i32>, #[case] expected: bool) {
        let actual = Solution::unique_occurrences(arr);
        assert_eq!(actual, expected);
    }
}
