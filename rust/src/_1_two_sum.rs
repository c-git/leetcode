//! Solution for https://leetcode.com/problems/two-sum
//! 1. Two Sum

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        todo!("Fill in body")
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![2,7,11,15], 9, vec![0,1])]
    #[case(vec![3,2,4], 6, vec![1,2])]
    #[case(vec![3,3], 6, vec![0,1])]
    fn case(#[case] nums: Vec<i32>, #[case] target: i32, #[case] mut expected: Vec<i32>) {
        let mut actual = Solution::two_sum(nums, target);
        // Sorts added because any order is correct
        actual.sort();
        expected.sort();
        assert_eq!(actual, expected);
    }
}
