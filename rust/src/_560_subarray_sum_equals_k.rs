//! Solution for https://leetcode.com/problems/subarray-sum-equals-k
//! 560. Subarray Sum Equals K

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;
        let mut curr_sum = 0;
        let mut left = 0;
        for (right, right_val) in nums.iter().enumerate() {
            curr_sum += right_val;
            if curr_sum == k {
                result += 1;
            }
            while left < right && curr_sum > k {
                curr_sum -= nums[left];
                if curr_sum == k {
                    result += 1;
                }
                left += 1;
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
    #[case(vec![1,1,1], 2, 2)]
    #[case(vec![1,2,3], 3, 2)]
    #[case(vec![1], 0, 0)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::subarray_sum(nums, k);
        assert_eq!(actual, expected);
    }
}
