//! Solution for https://leetcode.com/problems/largest-divisible-subset
//! 368. Largest Divisible Subset

// Longest sequence starting at a position
#[derive(Clone, Copy)]
struct SubsetInfo {
    next_idx: Option<usize>,
    length: usize,
}

impl Solution {
    // Based on https://www.youtube.com/watch?v=O-aXzrDB49w
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();
        let mut dp: Vec<SubsetInfo> = vec![
            SubsetInfo {
                next_idx: None,
                length: 1
            };
            nums.len()
        ];
        let mut result_start_idx = nums.len() - 1;

        for smaller_factor_idx in (0..nums.len()).rev() {
            for larger_factor_idx in smaller_factor_idx + 1..nums.len() {
                if nums[larger_factor_idx] % nums[smaller_factor_idx] == 0 {
                    if dp[smaller_factor_idx].length < dp[larger_factor_idx].length + 1 {
                        dp[smaller_factor_idx].length = dp[larger_factor_idx].length + 1;
                        dp[smaller_factor_idx].next_idx = Some(larger_factor_idx);
                        if dp[smaller_factor_idx].length > dp[result_start_idx].length {
                            result_start_idx = smaller_factor_idx;
                        }
                    }
                }
            }
        }

        let mut result = Vec::with_capacity(dp[result_start_idx].length);
        let mut next_idx_opt = Some(result_start_idx);
        while let Some(nex_idx) = next_idx_opt {
            result.push(nums[nex_idx]);
            next_idx_opt = dp[nex_idx].next_idx;
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
    #[case(vec![1,2,3], vec![1,2])]
    #[case(vec![1,2,4,8], vec![1,2,4,8])]
    #[case(vec![5,9,18,54,108,540,90,180,360,720], vec![9,18,90,180,360,720])]
    fn case(#[case] nums: Vec<i32>, #[case] expected: Vec<i32>) {
        let actual = Solution::largest_divisible_subset(nums);
        assert_eq!(actual, expected);
    }
}
