//! Solution for https://leetcode.com/problems/count-subarrays-where-max-element-appears-at-least-k-times
//! 2962. Count Subarrays Where Max Element Appears at Least K Times

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let mut result = 0;
        let max = nums
            .iter()
            .max()
            .expect("guaranteed to not be empty array in question");
        let mut count = 0;
        let mut left = 0;
        for (right, num) in nums.iter().enumerate() {
            if num == max {
                count += 1;
            }
            while count >= k {
                result += (nums.len() - right) as i64;
                if &nums[left] == max {
                    count -= 1;
                }
                left += 1;
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
    #[case(vec![1,3,2,3,3], 2, 6)]
    #[case(vec![1,4,2,1], 3, 0)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i64) {
        let actual = Solution::count_subarrays(nums, k);
        assert_eq!(actual, expected);
    }
}
