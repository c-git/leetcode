//! Solution for https://leetcode.com/problems/combination-sum-iv
//! 377. Combination Sum IV

impl Solution {
    pub fn combination_sum4(mut nums: Vec<i32>, target: i32) -> i32 {
        // Based on https://leetcode.com/problems/combination-sum-iv/solutions/4020255/rust-0ms-python-50ms-dynamic-programming-with-explanation/
        let mut dp = vec![0; target as usize + 1];
        dp[0] = 1;

        nums.sort_unstable();
        for sub_target in 1..=target {
            for &num in nums.iter() {
                let diff = sub_target - num;
                if diff < 0 {
                    break;
                }
                dp[sub_target as usize] += dp[diff as usize];
            }
        }

        *dp.last().unwrap()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,3], 4, 7)]
    #[case(vec![1,2], 4, 5)]
    #[case(vec![1], 4, 1)]
    #[case(vec![4], 4, 1)]
    #[case(vec![1,4], 4, 2)]
    #[case(vec![9], 3, 0)]
    fn case(#[case] nums: Vec<i32>, #[case] target: i32, #[case] expected: i32) {
        let actual = Solution::combination_sum4(nums, target);
        assert_eq!(actual, expected);
    }
}
