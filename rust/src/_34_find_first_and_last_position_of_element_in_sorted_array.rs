//! Solution for https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array
//! 34. Find First and Last Position of Element in Sorted Array

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let start = nums.partition_point(|&x| x < target);
        if start >= nums.len() || nums[start] != target {
            return vec![-1, -1];
        }
        let end = nums.partition_point(|&x| x <= target) - 1;
        vec![start as _, end as _]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![5,7,7,8,8,10], 8, vec![3,4])]
    #[case(vec![5,7,7,8,8,10], 6, vec![-1,-1])]
    #[case(vec![], 0, vec![-1,-1])]
    fn case(#[case] nums: Vec<i32>, #[case] target: i32, #[case] expected: Vec<i32>) {
        let actual = Solution::search_range(nums, target);
        assert_eq!(actual, expected);
    }
}
