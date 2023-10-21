//! Solution for https://leetcode.com/problems/constrained-subsequence-sum
//! 1425. Constrained Subsequence Sum

use std::collections::VecDeque;

impl Solution {
    pub fn constrained_subset_sum(nums: Vec<i32>, k: i32) -> i32 {
        // After reading editorial (Solution 3)
        let mut queue: VecDeque<i32> = VecDeque::new();
        let mut dp = vec![0; nums.len()];

        for i in 0..nums.len() {
            if !queue.is_empty() && i as i32 - queue[0] > k {
                queue.pop_front();
            }

            dp[i] = if queue.is_empty() {
                0
            } else {
                dp[queue[0] as usize]
            } + nums[i];

            while !queue.is_empty() && dp[*queue.back().unwrap() as usize] < dp[i] {
                queue.pop_back();
            }

            if dp[i] > 0 {
                queue.push_back(i as i32);
            }
        }
        dp.into_iter().max().unwrap()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![10,2,-10,5,20], 2, 37)]
    #[case(vec![-1,-2,-3], 1, -1)]
    #[case(vec![10,-2,-10,-5,20], 2, 23)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::constrained_subset_sum(nums, k);
        assert_eq!(actual, expected);
    }
}
