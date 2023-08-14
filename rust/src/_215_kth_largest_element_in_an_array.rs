//! Solution for https://leetcode.com/problems/kth-largest-element-in-an-array
//! 215. Kth Largest Element in an Array

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        // Uses two loops to avoid a branch inside of the loop
        let mut heap = BinaryHeap::with_capacity(k as usize + 1);

        // Load with k items
        for &num in nums.iter().take(k as usize) {
            heap.push(Reverse(num));
        }

        // Remove to make space then insert new until all items have been seen
        for &num in nums.iter().skip(k as usize) {
            heap.push(Reverse(num));
            heap.pop();
        }

        heap.pop().expect("Constraints say vec is non empty").0
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![3,2,1,5,6,4], 2, 5)]
    #[case(vec![3,2,3,1,2,4,5,5,6], 4, 4)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::find_kth_largest(nums, k);
        assert_eq!(actual, expected);
    }
}
