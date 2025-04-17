//! Solution for https://leetcode.com/problems/count-equal-and-divisible-pairs-in-an-array
//! 2176. Count Equal and Divisible Pairs in an Array

use std::collections::HashMap;

impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut result = 0;
        let mut seen: HashMap<i32, Vec<usize>> = HashMap::new();
        for (i, num) in nums.into_iter().enumerate() {
            let entry = seen.entry(num).or_default();
            for prev_idx in entry.iter().copied() {
                if (prev_idx * i) % k == 0 {
                    result += 1;
                }
            }
            entry.push(i);
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
    #[case(vec![3,1,2,2,2,1,3], 2, 4)]
    #[case(vec![1,2,3,4], 1, 0)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::count_pairs(nums, k);
        assert_eq!(actual, expected);
    }
}
