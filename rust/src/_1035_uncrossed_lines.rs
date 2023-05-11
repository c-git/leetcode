#![allow(clippy::needless_range_loop)]

use std::mem;
impl Solution {
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        // After reading editorial

        let n1 = nums1.len();
        let n2 = nums2.len();
        debug_assert_ne!(n1, 0);
        debug_assert_ne!(n2, 0);

        let mut dp = vec![0; n2 + 1]; // values for current row (n2 cols)
        let mut dp_prev = vec![0; n2 + 1]; // values for previous row (n2 cols)

        // Fill rest of cases
        for row in 1..=n1 {
            for col in 1..=n2 {
                dp[col] = if nums1[row - 1] == nums2[col - 1] {
                    dp_prev[col - 1] + 1
                } else {
                    dp[col - 1].max(dp_prev[col])
                };
                if cfg!(debug_assertions) {
                    println!("({row}, {col}): dp:      {dp:?}");
                    println!("({row}, {col}): dp_prev: {dp_prev:?}");
                    println!();
                }
            }
            mem::swap(&mut dp, &mut dp_prev);
        }
        dp_prev[n2]
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums1 = vec![1, 4, 2];
        let nums2 = vec![1, 2, 4];
        let expected = 2;
        let actual = Solution::max_uncrossed_lines(nums1, nums2);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case2() {
        let nums1 = vec![2, 5, 1, 2, 5];
        let nums2 = vec![10, 5, 2, 1, 5, 2];
        let expected = 3;
        let actual = Solution::max_uncrossed_lines(nums1, nums2);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case3() {
        let nums1 = vec![1, 3, 7, 1, 7, 5];
        let nums2 = vec![1, 9, 2, 5, 1];
        let expected = 2;
        let actual = Solution::max_uncrossed_lines(nums1, nums2);
        assert_eq!(actual, expected);
    }
}
