//! Solution for https://leetcode.com/problems/maximum-product-of-first-and-last-elements-of-a-subsequence
//! 3584. Maximum Product of First and Last Elements of a Subsequence

impl Solution {
    pub fn maximum_product(nums: Vec<i32>, m: i32) -> i64 {
        let n = nums.len();
        let m = m as usize;
        let nums: Vec<i64> = nums.into_iter().map(|x| x as i64).collect();
        let mut result = i64::MIN;
        for (first_idx, first) in nums.iter().enumerate().take(n - m + 1) {
            for last in nums.iter().skip(first_idx + m - 1) {
                result = result.max(first * last);
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
    #[case(vec![-1,-9,2,3,-2,-3,1], 1, 81)]
    #[case(vec![1,3,-5,5,6,-4], 3, 20)]
    #[case(vec![2,-1,2,-6,5,2,-5,7], 2, 35)]
    fn case(#[case] nums: Vec<i32>, #[case] m: i32, #[case] expected: i64) {
        let actual = Solution::maximum_product(nums, m);
        assert_eq!(actual, expected);
    }
}
