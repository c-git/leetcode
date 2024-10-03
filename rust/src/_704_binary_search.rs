//! Solution for https://leetcode.com/problems/binary-search
//! 704. Binary Search

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        nums.binary_search(&target).map(|x| x as i32).unwrap_or(-1)
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![-1,0,3,5,9,12], 9, 4)]
    #[case(vec![-1,0,3,5,9,12], 2, -1)]
    fn case(#[case] nums: Vec<i32>, #[case] target: i32, #[case] expected: i32) {
        let actual = Solution::search(nums, target);
        assert_eq!(actual, expected);
    }
}
