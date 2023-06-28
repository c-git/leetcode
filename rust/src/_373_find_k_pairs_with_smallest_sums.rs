//! Solution for https://leetcode.com/problems/find-k-pairs-with-smallest-sums
//! 373. Find K Pairs with Smallest Sums

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, mut k: i32) -> Vec<Vec<i32>> {
        // After reading editorial
        let mut result = vec![];
        let len1 = nums1.len();
        let len2 = nums2.len();

        let mut visited = HashSet::new();

        let mut heap = BinaryHeap::from([Reverse((nums1[0] + nums2[0], 0, 0))]);
        visited.insert((0, 0));

        while k > 0 && !heap.is_empty() {
            let Reverse((_sum, index1, index2)) = heap.pop().unwrap();
            result.push(vec![nums1[index1], nums2[index2]]);
            if index1 + 1 < len1 {
                push_pair(index1 + 1, index2, &mut visited, &mut heap, &nums1, &nums2);
            }
            if index2 + 1 < len2 {
                push_pair(index1, index2 + 1, &mut visited, &mut heap, &nums1, &nums2);
            }
            k -= 1;
        }
        result
    }
}

#[inline]
fn push_pair(
    a: usize,
    b: usize,
    visited: &mut HashSet<(usize, usize)>,
    heap: &mut BinaryHeap<Reverse<(i32, usize, usize)>>,
    nums1: &[i32],
    nums2: &[i32],
) {
    if !visited.contains(&(a, b)) {
        heap.push(Reverse((nums1[a] + nums2[b], a, b)));
        visited.insert((a, b));
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,7,11], vec![2,4,6], 3, vec![vec![1,2],vec![1,4],vec![1,6]])]
    #[case(vec![1,1,2], vec![1,2,3], 2, vec![vec![1,1],vec![1,1]])]
    #[case(vec![1,2], vec![3], 3, vec![vec![1,3],vec![2,3]])]
    #[case(vec![1,1,2], vec![1,2,3], 10, vec![vec![1,1],vec![1,1],vec![2,1],vec![1,2],vec![1,2],vec![2,2],vec![1,3],vec![1,3],vec![2,3]])]
    fn case(
        #[case] nums1: Vec<i32>,
        #[case] nums2: Vec<i32>,
        #[case] k: i32,
        #[case] mut expected: Vec<Vec<i32>>,
    ) {
        let mut actual = Solution::k_smallest_pairs(nums1, nums2, k);
        actual.sort();
        expected.sort();
        assert_eq!(actual, expected);
    }
}
