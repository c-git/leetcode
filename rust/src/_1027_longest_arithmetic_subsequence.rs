//! Solution for https://leetcode.com/problems/longest-arithmetic-subsequence
//! 1027. Longest Arithmetic Subsequence

impl Solution {
    /// From fastest solutions (showed 37ms, ran it and got 33ms)
    /// And 2MB memory which beats 100%
    /// After thinking about it and looking at the constraints it uses memory
    /// proportional to the max value which is 500 while the other one uses n^2 memory
    /// where max n is 1000
    ///
    /// let n <= 1000, let val <= 500
    /// Editorial runtime was O(n^2)
    /// Runtime for the fastest solution is O(n * val) so it is faster because of the restriction on val
    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        let (min, max) = nums
            .iter()
            .fold((i32::MAX, i32::MIN), |(min_val, max_val), &val| {
                (min_val.min(val), max_val.max(val))
            });
        let max_diff = max - min;
        let mut result = 2;
        let mut dp = vec![0_i32; max as usize + 1]; // Pull out creation here to reuse allocation
        for step in -max_diff..=max_diff {
            dp.iter_mut().for_each(|x| *x = 0); // Reset all values to 0
            for &x in &nums {
                let idx = x as usize;
                let previous_number_in_seq = x - step;
                let prev_len = dp.get(previous_number_in_seq as usize);
                dp[idx] = dp[idx].max(prev_len.copied().unwrap_or(0) + 1);
            }
            result = result.max(*dp.iter().max().unwrap());
        }
        result
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
    #[case(vec![0,500,0], 2)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::longest_arith_seq_length(nums);
        assert_eq!(actual, expected);
    }
}
