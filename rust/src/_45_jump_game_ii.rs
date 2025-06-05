//! Solution for https://leetcode.com/problems/jump-game-ii
//! 45. Jump Game II

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![0; n];

        for (i, range) in nums.into_iter().enumerate().rev().skip(1) {
            dp[i] = dp
                .iter()
                .skip(i + 1)
                .take(range as usize)
                .min()
                .copied()
                .unwrap_or(i32::MAX)
                .saturating_add(1);
        }
        dp[0]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![2,3,1,1,4], 2)]
    #[case(vec![2,3,0,1,4], 2)]
    #[case(vec![1], 0)]
    #[case(vec![1,2,3], 2)]
    #[case(vec![5,9,3,2,1,0,2,3,3,1,0,0], 3)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::jump(nums);
        assert_eq!(actual, expected);
    }
}
