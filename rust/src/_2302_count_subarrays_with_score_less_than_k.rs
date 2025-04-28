//! Solution for https://leetcode.com/problems/count-subarrays-with-score-less-than-k
//! 2302. Count Subarrays With Score Less Than K

impl Solution {
    /// Based on https://www.youtube.com/watch?v=CtLsKfvG06s
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        let mut result = 0;
        let mut right = 0;
        let mut sum = 0; // Sum of left..right
        for left in 0..nums.len() {
            while right < nums.len() && (sum + nums[right] as i64) * ((right - left + 1) as i64) < k
            {
                sum += nums[right] as i64;
                right += 1;
            }
            result += (right - left) as i64;

            // Slide window forward
            if right == left {
                // Window is empty move right forward as well
                right += 1;
            } else {
                sum -= nums[left] as i64;
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
