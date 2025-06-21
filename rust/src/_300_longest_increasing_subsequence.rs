//! Solution for https://leetcode.com/problems/longest-increasing-subsequence
//! 300. Longest Increasing Subsequence

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        // Stores the max length of a sub sequence ending at that index
        let mut dp = vec![1; nums.len()];

        for (curr_idx, num) in nums.iter().enumerate() {
            for (prev_idx, prev) in nums.iter().enumerate().take(curr_idx) {
                if prev < num {
                    dp[curr_idx] = dp[curr_idx].max(dp[prev_idx] + 1);
                }
            }
        }
        *dp.iter().max().unwrap()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![10,9,2,5,3,7,101,18], 4)]
    #[case(vec![0,1,0,3,2,3], 4)]
    #[case(vec![7,7,7,7,7,7,7], 1)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::length_of_lis(nums);
        assert_eq!(actual, expected);
    }
}
