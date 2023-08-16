//! Solution for https://leetcode.com/problems/sliding-window-maximum
//! 239. Sliding Window Maximum

use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;

        let mut result = Vec::with_capacity(nums.len() + 1 - k);
        let mut window = VecDeque::with_capacity(k);

        // Fill window with first k items
        for &num in nums.iter().take(k) {
            window.push_back(num);
        }

        // Save max for first
        let mut window_max = *window.iter().max().unwrap();
        result.push(window_max);

        // Move windows to the right and find what is the newest value to add
        for &num in nums.iter().skip(k) {
            let oldest = window.pop_front().unwrap(); // Remove oldest
            window.push_back(num); // Add next value
            match (num >= window_max, window_max == oldest) {
                (true, _) => {
                    window_max = num;
                }
                (false, true) => {
                    // Max moved out of window, find new max
                    window_max = *window.iter().max().unwrap();
                }
                (false, false) => {} // Do nothing, the max is still in the window and nothing bigger has come along
            }
            result.push(window_max);
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
    #[case(vec![1,3,-1,-3,5,3,6,7], 3, vec![3,3,5,5,6,7])]
    #[case(vec![1], 1, vec![1])]
    #[case(vec![1,1,1,1,1,1,1,1], 3, vec![1,1,1,1,1,1])]
    #[case(vec![8,7,6,5,4,3,2,1], 3, vec![8,7,6,5,4,3])]
    #[case(vec![1,2,3,4,5,6,7,8], 3, vec![3,4,5,6,7,8])]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: Vec<i32>) {
        let actual = Solution::max_sliding_window(nums, k);
        assert_eq!(actual, expected);
    }
}
