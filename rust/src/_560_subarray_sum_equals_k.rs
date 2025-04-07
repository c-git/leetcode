//! Solution for https://leetcode.com/problems/subarray-sum-equals-k
//! 560. Subarray Sum Equals K

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let prefix_sum = {
            // Build prefix sums
            let mut sum = 0;
            let mut res = Vec::with_capacity(nums.len());
            for num in nums.iter() {
                sum += num;
                res.push(sum);
            }
            res
        };

        let mut result = 0;

        for (right, &right_val) in prefix_sum.iter().enumerate() {
            if right_val == k {
                result += 1;
            }
            for left_val in prefix_sum.iter().take(right) {
                if right_val - left_val == k {
                    result += 1;
                }
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
    #[case(vec![-1,-1,1], 0, 1)]
    #[case(vec![28,54,7,-70,22,65,-6], 100, 1)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::subarray_sum(nums, k);
        assert_eq!(actual, expected);
    }
}
