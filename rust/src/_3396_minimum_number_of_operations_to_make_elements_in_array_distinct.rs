//! Solution for https://leetcode.com/problems/minimum-number-of-operations-to-make-elements-in-array-distinct
//! 3396. Minimum Number of Operations to Make Elements in Array Distinct

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut seen = std::collections::HashSet::new();
        for (i, num) in nums.iter().enumerate().rev() {
            if seen.contains(num) {
                return (i + 1).div_ceil(3) as i32;
            }
            seen.insert(*num);
        }
        0
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,3,4,2,3,3,5,7], 2)]
    #[case(vec![4,5,6,4,4], 2)]
    #[case(vec![6,7,8,9], 0)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::minimum_operations(nums);
        assert_eq!(actual, expected);
    }
}
