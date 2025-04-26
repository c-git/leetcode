//! Solution for https://leetcode.com/problems/count-subarrays-with-fixed-bounds
//! 2444. Count Subarrays With Fixed Bounds

impl Solution {
    /// Based on https://www.youtube.com/watch?v=Bk-HxzaooqM
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut result = 0;
        let mut bad_i = -1;
        let mut min_i = -1;
        let mut max_i = -1;
        let n = nums.len() as i32;
        for i in 0..n {
            let num = nums[i as usize];
            if num < min_k || num > max_k {
                bad_i = i
            }
            if num == min_k {
                min_i = i
            }
            if num == max_k {
                max_i = i
            }
            result += std::cmp::max(0, std::cmp::min(min_i, max_i) - bad_i) as i64;
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
    #[case(vec![1,3,5,2,7,5], 1, 5, 2)]
    #[case(vec![1,1,1,1], 1, 1, 10)]
    fn case(#[case] nums: Vec<i32>, #[case] min_k: i32, #[case] max_k: i32, #[case] expected: i64) {
        let actual = Solution::count_subarrays(nums, min_k, max_k);
        assert_eq!(actual, expected);
    }
}
