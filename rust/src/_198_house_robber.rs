//! Solution for https://leetcode.com/problems/house-robber
//! 198. House Robber

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        // Each index stores the best value possible up to that point
        let mut dp = vec![0; nums.len()];
        dp[0] = nums[0];
        dp[1] = nums[0].max(nums[1]);
        for i in 2..nums.len() {
            dp[i] = dp[i - 1].max(dp[i - 2] + nums[i]);
        }
        *dp.last().unwrap()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,3,1], 4)]
    #[case(vec![2,7,9,3,1], 12)]
    #[case(vec![2,1,1,2], 4)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::rob(nums);
        assert_eq!(actual, expected);
    }
}
