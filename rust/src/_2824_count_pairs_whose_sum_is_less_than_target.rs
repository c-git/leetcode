//! Solution for https://leetcode.com/problems/count-pairs-whose-sum-is-less-than-target
//! 2824. Count Pairs Whose Sum is Less than Target

impl Solution {
    pub fn count_pairs(mut nums: Vec<i32>, target: i32) -> i32 {
        // Based on Beixuan's Idea
        nums.sort_unstable();
        let mut left = 0;
        let mut right = nums.len() - 1;
        let mut result = 0;
        while left < right {
            if nums[left] + nums[right] < target {
                result += right - left;
                left += 1;
            } else {
                right -= 1;
            }
        }
        result as _
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![-1,1,2,3,1], 2, 3)]
    #[case(vec![-6,2,5,-2,-7,-1,3], -2, 10)]
    fn case(#[case] nums: Vec<i32>, #[case] target: i32, #[case] expected: i32) {
        let actual = Solution::count_pairs(nums, target);
        assert_eq!(actual, expected);
    }
}
