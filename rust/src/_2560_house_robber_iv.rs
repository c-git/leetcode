//! Solution for https://leetcode.com/problems/house-robber-iv
//! 2560. House Robber IV

impl Solution {
    /// Based on editorial
    pub fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
        // Store the maximum nums value in maxReward.
        let (mut min_reward, mut max_reward) = (
            1,
            *nums
                .iter()
                .max()
                .expect("guaranteed to have at least one by problem"),
        );
        let total_houses = nums.len();

        // Use binary search to find the minimum reward possible.
        while min_reward < max_reward {
            let mid_reward = (min_reward + max_reward) / 2;
            let mut possible_thefts = 0;

            let mut index = 0;
            while index < total_houses {
                if nums[index] <= mid_reward {
                    possible_thefts += 1;
                    index += 2 // Skip the next house to maintain the non-adjacent condition
                } else {
                    index += 1
                }
            }
            if possible_thefts >= k {
                max_reward = mid_reward
            } else {
                min_reward = mid_reward + 1
            }
        }
        min_reward
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![2,3,5,9], 2, 5)]
    #[case(vec![2,7,9,3,1], 2, 2)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::min_capability(nums, k);
        assert_eq!(actual, expected);
    }
}
