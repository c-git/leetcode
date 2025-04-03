//! Solution for https://leetcode.com/problems/single-number
//! 136. Single Number

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, |acc, x| acc ^ x)
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![2,2,1], 1)]
    #[case(vec![4,1,2,1,2], 4)]
    #[case(vec![1], 1)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::single_number(nums);
        assert_eq!(actual, expected);
    }
}
