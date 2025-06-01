//! Solution for https://leetcode.com/problems/top-k-frequent-elements
//! 347. Top K Frequent Elements

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let freq_count = nums
            .iter()
            .fold(HashMap::<i32, usize>::new(), |mut acc, num| {
                *acc.entry(*num).or_default() += 1;
                acc
            });
        let most_freq = freq_count
            .into_iter()
            .fold(BinaryHeap::new(), |mut heap, (num, freq)| {
                heap.push(Reverse((freq, num)));
                if heap.len() > k {
                    heap.pop();
                }
                heap
            });
        most_freq
            .into_iter()
            .map(|Reverse((_freq, num))| num)
            .collect()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,1,1,2,2,3], 2, vec![1,2])]
    #[case(vec![1], 1, vec![1])]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] mut expected: Vec<i32>) {
        let mut actual = Solution::top_k_frequent(nums, k);
        actual.sort();
        expected.sort();
        assert_eq!(actual, expected);
    }
}
