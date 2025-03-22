//! Solution for https://leetcode.com/problems/minimum-operations-to-make-binary-array-elements-equal-to-one-i
//! 3191. Minimum Operations to Make Binary Array Elements Equal to One I

impl Solution {
    /// Based on editorial
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for i in 2..nums.len() {
            if nums[i - 2] == 0 {
                result += 1;
                Self::flip(&mut nums[i - 2..=i])
            }
        }

        if nums.len() as i32 == nums.iter().sum::<i32>() {
            result
        } else {
            -1
        }
    }

    fn flip(nums: &mut [i32]) {
        for x in nums {
            *x = if *x == 0 { 1 } else { 0 };
        }
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![0,1,1,1,0,0], 3)]
    #[case(vec![0,1,1,1], -1)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::min_operations(nums);
        assert_eq!(actual, expected);
    }
}
