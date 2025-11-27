//! Solution for https://leetcode.com/problems/maximum-subarray-sum-with-length-divisible-by-k
//! 3381. Maximum Subarray Sum With Length Divisible by K

impl Solution {
    /// Based on https://www.youtube.com/watch?v=8rwW3iKqP34
    pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut result = i64::MIN;
        let mut min_prefix_of_len = vec![i64::MAX; k];
        min_prefix_of_len[0] = 0;
        let mut total = 0;
        for (i, &num) in nums.iter().enumerate() {
            total += num as i64;
            let length = i + 1;
            let prefix_length = length % k;
            result = result.max(total.saturating_sub(min_prefix_of_len[prefix_length]));
            min_prefix_of_len[prefix_length] = min_prefix_of_len[prefix_length].min(total);
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
    #[case(vec![1,2], 1, 3)]
    #[case(vec![-1,-2,-3,-4,-5], 4, -10)]
    #[case(vec![-5,1,2,-3,4], 2, 4)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i64) {
        let actual = Solution::max_subarray_sum(nums, k);
        assert_eq!(actual, expected);
    }
}
