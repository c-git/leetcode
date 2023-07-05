//! Solution for https://leetcode.com/problems/maximum-subarray
//! 53. Maximum Subarray

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut cumulative_sums = Vec::with_capacity(n);
        cumulative_sums.push(nums[0]);
        for i in 1..n {
            let last_sum = cumulative_sums[i - 1];
            cumulative_sums.push(last_sum + nums[i]);
        }

        let mut result = cumulative_sums[0];
        for right in 1..n {
            result = result.max(cumulative_sums[right]);
            for left in 0..right {
                result = result.max(cumulative_sums[right] - cumulative_sums[left]);
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
