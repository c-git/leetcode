use std::collections::HashSet;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let nums1: HashSet<_> = nums1.into_iter().collect();
        let nums2: HashSet<_> = nums2.into_iter().collect();
        nums1.intersection(&nums2).copied().collect()
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        let expected = vec![2];
        let actual = Solution::intersection(nums1, nums2);
        validator(actual, expected);
    }

    #[test]
    fn case2() {
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];
        let expected = vec![9, 4];
        let actual = Solution::intersection(nums1, nums2);
        validator(actual, expected);
    }

    fn validator(mut actual: Vec<i32>, mut expected: Vec<i32>) {
        actual.sort();
        expected.sort();
        assert_eq!(actual, expected);
    }
}
