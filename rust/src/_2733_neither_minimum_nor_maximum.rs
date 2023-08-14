//! Solution for https://leetcode.com/problems/neither-minimum-nor-maximum
//! 2733. Neither Minimum nor Maximum

impl Solution {
    pub fn find_non_min_or_max(mut nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return -1; // Not enough values to not be min or max
        }
        let relevant_nums = &mut nums[0..=2];
        relevant_nums.sort_unstable();
        relevant_nums[1]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![3,2,1,4], 2)]
    #[case(vec![1,2], -1)]
    #[case(vec![2,1,3], 2)]
    #[case(vec![2,40,8,1], 8)] // Both 2 and 8 are correct by my algorithm only looks at first 3
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::find_non_min_or_max(nums);
        assert_eq!(actual, expected);
    }
}
