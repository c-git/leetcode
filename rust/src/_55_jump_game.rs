//! Solution for https://leetcode.com/problems/jump-game
//! 55. Jump Game

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut dp = vec![false; n];
        *dp.last_mut().unwrap() = true;
        for i in (0..n - 1).rev() {
            for jump in 1..=nums[i] as usize {
                let other = jump + i;
                debug_assert!(
                    other < n,
                    "If we had already reached n the loop should have broken"
                );
                if dp[other] {
                    // We've found a path to the end
                    dp[i] = true;
                    break;
                }
            }
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
    #[case(vec![2,3,1,1,4], true)]
    #[case(vec![3,2,1,0,4], false)]
    #[case(vec![3,2,1,1,0], true)]
    #[case(vec![0], true)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: bool) {
        let actual = Solution::can_jump(nums);
        assert_eq!(actual, expected);
    }
}
