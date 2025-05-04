//! Solution for https://leetcode.com/problems/number-of-equivalent-domino-pairs
//! 1128. Number of Equivalent Domino Pairs

use std::collections::HashMap;

impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let mut seen: HashMap<Vec<i32>, i32> = HashMap::new();
        for mut domino in dominoes {
            domino.sort();
            let entry = seen.entry(domino).or_default();
            result += *entry;
            *entry += 1;
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
    #[case(vec![vec![1,2],vec![2,1],vec![3,4],vec![5,6]], 1)]
    #[case(vec![vec![1,2],vec![1,2],vec![1,1],vec![1,2],vec![2,2]], 3)]
    fn case(#[case] dominoes: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::num_equiv_domino_pairs(dominoes);
        assert_eq!(actual, expected);
    }
}
