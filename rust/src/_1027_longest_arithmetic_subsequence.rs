//! Solution for https://leetcode.com/problems/longest-arithmetic-subsequence
//! 1027. Longest Arithmetic Subsequence

impl Solution {
    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        // From fastest solutions (showed 37ms, ran it and got 33ms)
        // And 2MB memory which beats 100%
        // After thinking about it and looking at the constraints it uses memory
        // proportional to the max value which is 500 while the other one uses n^2 memory
        // where max n is 1000
        let max = *nums.iter().max().unwrap();
        let mut result = Vec::with_capacity(max as usize * 2 + 1);
        let mut dp = vec![0_i32; max as usize + 1]; // Pull out creation here to reuse allocation
        for step in -max..=max {
            dp.iter_mut().for_each(|x| *x = 0); // Reset all values to 0
            for &x in &nums {
                let idx = x as usize;
                dp[idx] = dp[idx].max(dp.get((x - step) as usize).copied().unwrap_or(0) + 1);
            }
            result.push(*dp.iter().max().unwrap());
        }
        *result.iter().max().unwrap()
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
