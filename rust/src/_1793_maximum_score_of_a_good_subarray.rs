//! Solution for https://leetcode.com/problems/maximum-score-of-a-good-subarray
//! 1793. Maximum Score of a Good Subarray

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        // Source: https://leetcode.com/problems/maximum-score-of-a-good-subarray/solutions/4194071/92-13-two-pointers/
        let mut left = k as usize;
        let mut right = k as usize;
        let mut min_val = nums[k as usize];
        let mut max_score = min_val;

        while left > 0 || right < nums.len() - 1 {
            if left == 0 || (right < nums.len() - 1 && nums[right + 1] > nums[left - 1]) {
                right += 1;
            } else {
                left -= 1;
            }
            min_val = min_val.min(nums[left].min(nums[right]));
            max_score = max_score.max(min_val * (right as i32 - left as i32 + 1));
        }

        max_score
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,4,3,7,4,5], 3, 15)]
    #[case(vec![5,5,4,5,4,1,1,1], 0, 20)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::maximum_score(nums, k);
        assert_eq!(actual, expected);
    }
}
