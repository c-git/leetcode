//! Solution for https://leetcode.com/problems/max-consecutive-ones-iii
//! 1004. Max Consecutive Ones III

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut left_idx = 0;
        let mut result = 0;
        let mut zeros = 0;
        for right_idx in 0..nums.len() {
            if nums[right_idx] == 0 {
                zeros += 1;
                while zeros > k {
                    if nums[left_idx] == 0 {
                        zeros -= 1;
                    }
                    left_idx += 1;
                }
            }
            debug_assert!(left_idx <= right_idx);
            result = result.max(right_idx - left_idx + 1)
        }
        result as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,1,1,0,0,0,1,1,1,1,0], 2, 6)]
    #[case(vec![0,0,1,1,0,0,1,1,1,0,1,1,0,0,0,1,1,1,1], 3, 10)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::longest_ones(nums, k);
        assert_eq!(actual, expected);
    }
}
