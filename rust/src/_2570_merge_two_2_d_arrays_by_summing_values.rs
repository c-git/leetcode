//! Solution for https://leetcode.com/problems/merge-two-2d-arrays-by-summing-values
//! 2570. Merge Two 2D Arrays by Summing Values

use std::cmp::Ordering;

impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = Vec::with_capacity(nums1.len().max(nums2.len()));
        let mut iter1 = nums1.into_iter();
        let mut iter2 = nums2.into_iter();
        let mut next1 = iter1.next();
        let mut next2 = iter2.next();

        // Main case we have values in both arrays still
        loop {
            (next1, next2) = match (next1, next2) {
                (None, None) => break,
                (None, Some(value)) | (Some(value), None) => {
                    result.push(value);
                    (iter1.next(), iter2.next())
                }
                (Some(mut n1), Some(n2)) => match n1[0].cmp(&n2[0]) {
                    Ordering::Less => {
                        result.push(n1);
                        (iter1.next(), Some(n2))
                    }
                    Ordering::Greater => {
                        result.push(n2);
                        (Some(n1), iter2.next())
                    }
                    Ordering::Equal => {
                        n1[1] += n2[1];
                        result.push(n1);
                        (iter1.next(), iter2.next())
                    }
                },
            };
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
    #[case(vec![vec![1,2],vec![2,3],vec![4,5]], vec![vec![1,4],vec![3,2],vec![4,1]], vec![vec![1,6],vec![2,3],vec![3,2],vec![4,6]])]
    #[case(vec![vec![2,4],vec![3,6],vec![5,5]], vec![vec![1,3],vec![4,3]], vec![vec![1,3],vec![2,4],vec![3,6],vec![4,3],vec![5,5]])]
    fn case(
        #[case] nums1: Vec<Vec<i32>>,
        #[case] nums2: Vec<Vec<i32>>,
        #[case] expected: Vec<Vec<i32>>,
    ) {
        let actual = Solution::merge_arrays(nums1, nums2);
        assert_eq!(actual, expected);
    }
}
