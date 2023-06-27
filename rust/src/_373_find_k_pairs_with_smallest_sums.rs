//! Solution for https://leetcode.com/problems/find-k-pairs-with-smallest-sums
//! 373. Find K Pairs with Smallest Sums

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut iter1 = nums1.into_iter().peekable();
        let mut iter2 = nums2.into_iter().peekable();
        let mut val1 = iter1.next().unwrap();
        let mut val2 = iter2.next().unwrap();
        for _ in 0..k {
            // LI:  - val1, val2 is the next pair of values to add
            //      -  There number of sum of items in iter1 and iter2 decreases each iteration
            result.push(vec![val1, val2]);
            match (iter1.peek(), iter2.peek()) {
                (None, None) => break,
                (None, Some(_)) => val2 = iter2.next().unwrap(),
                (Some(_), None) => val1 = iter1.next().unwrap(),
                (Some(v1), Some(v2)) => {
                    if v1 < v2 {
                        val1 = iter1.next().unwrap();
                    } else {
                        val2 = iter2.next().unwrap();
                    }
                }
            }
        }
        result
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
    fn case(
        #[case] nums1: Vec<i32>,
        #[case] nums2: Vec<i32>,
        #[case] k: i32,
        #[case] expected: Vec<Vec<i32>>,
    ) {
        let actual = Solution::k_smallest_pairs(nums1, nums2, k);
        assert_eq!(actual, expected);
    }
}
