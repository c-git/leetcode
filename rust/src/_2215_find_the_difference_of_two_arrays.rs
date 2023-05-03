use std::collections::BTreeSet;

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let nums1: BTreeSet<_> = nums1.into_iter().collect();
        let nums2: BTreeSet<_> = nums2.into_iter().collect();

        // Must use copy here because otherwise values point to the values from the local variables
        vec![
            nums1.difference(&nums2).copied().collect(),
            nums2.difference(&nums1).copied().collect(),
        ]
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn case1() {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![2, 4, 6];
        let expected = vec![vec![1, 3], vec![4, 6]];
        let actual = Solution::find_difference(nums1, nums2);
        validator(actual, expected);
    }

    #[test]
    fn case2() {
        let nums1 = vec![1, 2, 3, 3];
        let nums2 = vec![1, 1, 2, 2];
        let expected = vec![vec![3], vec![]];
        let actual = Solution::find_difference(nums1, nums2);
        validator(actual, expected);
    }

    fn validator(actual: Vec<Vec<i32>>, expected: Vec<Vec<i32>>) {
        assert_eq!(expected.len(), 2);
        assert_eq!(actual.len(), expected.len());

        let (a1, a2): (BTreeSet<_>, BTreeSet<_>) =
            (actual[0].iter().collect(), actual[1].iter().collect());
        let (e1, e2): (BTreeSet<_>, BTreeSet<_>) =
            (expected[0].iter().collect(), expected[1].iter().collect());

        assert_eq!(a1, e1);
        assert_eq!(a2, e2);
    }
}
