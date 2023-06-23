//! Solution for https://leetcode.com/problems/longest-arithmetic-subsequence
//! 1027. Longest Arithmetic Subsequence

// From solutions (showed 37ms running to compare and get memory usage)
impl Solution {
    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        let max = *nums.iter().max().unwrap();
        (-max..=max)
            .map(|step| {
                let mut dp = vec![0_i32; max as usize + 1];
                for &x in &nums {
                    dp[x as usize] =
                        dp[x as usize].max(dp.get((x - step) as usize).copied().unwrap_or(0) + 1);
                }
                *dp.iter().max().unwrap()
            })
            .max()
            .unwrap()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![3,6,9,12], 4)]
    #[case(vec![9,4,7,2,10], 3)]
    #[case(vec![20,1,15,3,10,5,8], 4)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::longest_arith_seq_length(nums);
        assert_eq!(actual, expected);
    }
}
