use std::collections::HashMap;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        // This solution is to simulate the ability only read one value from num2 at a time because it is too big and is stored on disk
        // See history in git for faster solution that requires sorting

        let (nums1, nums2) = if nums1.len() < nums2.len() {
            (nums1, nums2)
        } else {
            (nums2, nums1)
        };

        let mut lookup_set = HashMap::with_capacity(nums1.len());
        for num in nums1 {
            *lookup_set.entry(num).or_insert(0) += 1;
        }

        let mut result = vec![];

        for num2 in nums2 {
            if let Some(duplicity) = lookup_set.get_mut(&num2) {
                result.push(num2);
                debug_assert!(duplicity >= &mut 1);
                if duplicity == &1 {
                    lookup_set.remove(&num2);
                } else {
                    *duplicity -= 1;
                }
            }
        }
        result
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
        let expected = vec![2, 2];
        let actual = Solution::intersect(nums1, nums2);
        validator(actual, expected);
    }

    #[test]
    fn case2() {
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];
        let expected = vec![9, 4];
        let actual = Solution::intersect(nums1, nums2);
        validator(actual, expected);
    }

    fn validator(mut actual: Vec<i32>, mut expected: Vec<i32>) {
        actual.sort();
        expected.sort();
        assert_eq!(actual, expected);
    }
}
