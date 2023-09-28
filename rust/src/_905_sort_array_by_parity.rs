//! Solution for https://leetcode.com/problems/sort-array-by-parity
//! 905. Sort Array By Parity

impl Solution {
    pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable_by_key(|x| x % 2);
        nums
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![3,1,2,4], vec![2,4,3,1])]
    #[case(vec![0], vec![0])]
    fn case(#[case] nums: Vec<i32>, #[case] expected: Vec<i32>) {
        let actual = Solution::sort_array_by_parity(nums);
        assert_eq!(actual, expected);
    }
}
