//! Solution for https://leetcode.com/problems/house-robber-ii
//! 213. House Robber II

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        // Stores the best solution up to that index
        let mut dp_without_last = vec![0; nums.len()];
        let mut dp_without_first = dp_without_last.clone();
        dp_without_last[0] = nums[0]; // Allowed to use first because last will be ignored
        dp_without_last[1] = nums[1].max(nums[0]);
        dp_without_first[1] = nums[1]; // Will always be the second value because we cannot use first
        for i in 2..nums.len() {
            dp_without_first[i] = dp_without_first[i - 1].max(dp_without_first[i - 2] + nums[i]);
            dp_without_last[i] = dp_without_last[i - 1].max(dp_without_last[i - 2] + nums[i]);
        }

        dp_without_last[nums.len() - 2].max(*dp_without_first.last().unwrap())
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![2,3,2], 3)]
    #[case(vec![1,2,3,1], 4)]
    #[case(vec![1,2,3], 3)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::rob(nums);
        assert_eq!(actual, expected);
    }
}
