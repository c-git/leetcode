//! Solution for https://leetcode.com/problems/count-the-number-of-fair-pairs
//! 2563. Count the Number of Fair Pairs

impl Solution {
    /// Based on https://www.youtube.com/watch?v=TjthKf7Mc_8
    pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        let mut result = 0;
        nums.sort_unstable();
        for (i, num) in nums.iter().take(nums.len() - 1).enumerate() {
            let partition_start = lower - num;
            let partition_end = upper - num;
            let start = nums[i + 1..].partition_point(|&x| x < partition_start);
            // Uses <= to move past the equal value
            let end = nums[i + 1..].partition_point(|&x| x <= partition_end);
            result += (end - start) as i64;
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
    #[case(vec![0,1,7,4,4,5], 3, 6, 6)]
    #[case(vec![1,7,9,2,5], 11, 11, 1)]
    fn case(#[case] nums: Vec<i32>, #[case] lower: i32, #[case] upper: i32, #[case] expected: i64) {
        let actual = Solution::count_fair_pairs(nums, lower, upper);
        assert_eq!(actual, expected);
    }
}
