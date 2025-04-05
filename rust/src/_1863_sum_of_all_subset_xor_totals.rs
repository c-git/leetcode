//! Solution for https://leetcode.com/problems/sum-of-all-subset-xor-totals
//! 1863. Sum of All Subset XOR Totals

impl Solution {
    // Based on https://www.youtube.com/watch?v=HToBFhTa1uQ
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for num in nums.iter().copied() {
            result |= num;
        }
        result << (nums.len() - 1)
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,3], 6)]
    #[case(vec![5,1,6], 28)]
    #[case(vec![3,4,5,6,7,8], 480)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::subset_xor_sum(nums);
        assert_eq!(actual, expected);
    }
}
