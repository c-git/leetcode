//! Solution for https://leetcode.com/problems/minimum-operations-to-make-array-sum-divisible-by-k
//! 3512. Minimum Operations to Make Array Sum Divisible by K

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        nums.iter().sum::<i32>() % k
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![3,9,7], 5, 4)]
    #[case(vec![4,1,3], 4, 0)]
    #[case(vec![3,2], 6, 5)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::min_operations(nums, k);
        assert_eq!(actual, expected);
    }
}
