//! Solution for https://leetcode.com/problems/house-robber
//! 198. House Robber

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut taken = 0;
        let mut not_taken = 0;
        for num in nums {
            let temp = not_taken + num;
            not_taken = taken.max(not_taken);
            taken = temp;
        }
        taken.max(not_taken)
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,3,1], 4)]
    #[case(vec![2,7,9,3,1], 12)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::rob(nums);
        assert_eq!(actual, expected);
    }
}
