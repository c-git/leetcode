//! Solution for https://leetcode.com/problems/k-diff-pairs-in-an-array
//! 532. K-diff Pairs in an Array

impl Solution {
    pub fn find_pairs(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let mut result = 0;
        let mut left = 0;
        let mut right = 1;
        while right < nums.len() {
            if left >= right {
                right += 1;
                continue;
            }
            let diff = (nums[left] - nums[right]).abs();
            if diff == k {
                result += 1;
            }
            if diff >= k {
                left += 1;
                while left < nums.len() && nums[left - 1] == nums[left] {
                    left += 1;
                }
            } else {
                right += 1;
                while right < nums.len() && nums[right - 1] == nums[right] {
                    right += 1;
                }
            }
        }
        result
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![3,1,4,1,5], 2, 2)]
    #[case(vec![1,2,3,4,5], 1, 4)]
    #[case(vec![1,3,1,5,4], 0, 1)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::find_pairs(nums, k);
        assert_eq!(actual, expected);
    }
}
