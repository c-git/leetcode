//! Solution for https://leetcode.com/problems/count-subarrays-with-score-less-than-k
//! 2302. Count Subarrays With Score Less Than K

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        let mut result = 0;
        for start in 0..nums.len() {
            let mut sum = 0;
            let mut len = 0;
            for &num in nums.iter().skip(start) {
                sum += num as i64;
                len += 1;
                if sum * len < k {
                    result += 1;
                } else {
                    break;
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
    #[case(vec![2,1,4,3,5], 10, 6)]
    #[case(vec![1,1,1], 5, 5)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i64, #[case] expected: i64) {
        let actual = Solution::count_subarrays(nums, k);
        assert_eq!(actual, expected);
    }
}
