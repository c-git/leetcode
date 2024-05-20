//! Solution for https://leetcode.com/problems/sum-of-all-subset-xor-totals
//! 1863. Sum of All Subset XOR Totals

impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        // Translated version of editorial

        // Capture each bit that is set in any of the elements
        let result = nums.iter().fold(0, |acc, x| acc | x);

        // Multiply by the number of subset XOR totals that will have each bit set
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
