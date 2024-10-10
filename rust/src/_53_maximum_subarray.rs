//! Solution for https://leetcode.com/problems/maximum-subarray
//! 53. Maximum Subarray

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        // Source: https://leetcode.com/problems/maximum-subarray/solutions/3315652/rust-dynamic-programming-kadane-s-algorithm-concise-solution/
        let mut result = i32::MIN;
        let mut current_sum = 0i32;
        for num in nums {
            // Calculate value of extension
            current_sum = current_sum.saturating_add(num);

            // Take better of extended sum or restart
            current_sum = current_sum.max(num);

            // Update best seen
            result = result.max(current_sum);
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
    #[case(vec![-2,1,-3,4,-1,2,1,-5,4], 6)]
    #[case(vec![1], 1)]
    #[case(vec![5,4,-1,7,8], 23)]
    #[case(vec![-5,-4,-1,-7,8], 8)]
    #[case(vec![5,-4,-1,-7,-8], 5)]
    #[case(vec![-5,-4,1,-7,-8], 1)]
    #[case(vec![-5,-4,-1,-7,-8], -1)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::max_sub_array(nums);
        assert_eq!(actual, expected);
    }
}
