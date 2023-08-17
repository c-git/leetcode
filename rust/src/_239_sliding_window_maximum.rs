//! Solution for https://leetcode.com/problems/sliding-window-maximum
//! 239. Sliding Window Maximum

use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // Changed to solution from editorial (Based on description not actual translation of the code)
        let k = k as usize;

        let mut result = Vec::with_capacity(nums.len() + 1 - k);
        let mut relevant_window_items: VecDeque<(usize, i32)> = VecDeque::with_capacity(k); // Only the monotonically decreasing values from the current window

        // Fill with relevant items from the first k items
        for (i, num) in nums.iter().enumerate().take(k) {
            while let Some((_idx, last)) = relevant_window_items.back() {
                if last < num {
                    relevant_window_items.pop_back(); // Value in same window as num but it's smaller just remove it
                } else {
                    // Value bigger so it would be the max not this one so leave it in
                    break;
                }
            }
            relevant_window_items.push_back((i, *num));
        }

        // Save first max (the one at the front of the relevant window)
        let &(_idx, max) = relevant_window_items.front().unwrap();
        result.push(max);

        // Move windows to the right and find what is the newest value to add
        for (i, num) in nums.iter().enumerate().skip(k) {
            let (oldest_idx, _oldest_val) = relevant_window_items.front().unwrap();
            if *oldest_idx <= i.saturating_sub(k) {
                debug_assert_eq!(*oldest_idx, i.saturating_sub(k), "Only checked for less than to handle that case but should never happen as we check each iteration");
                relevant_window_items.pop_front(); // Too old remove from window
            }
            while let Some((_idx, last)) = relevant_window_items.back() {
                if last < num {
                    relevant_window_items.pop_back(); // Value in same window as num but it's smaller just remove it
                } else {
                    // Value bigger so it would be the max not this one so leave it in
                    break;
                }
            }
            relevant_window_items.push_back((i, *num)); // Add next value
            let &(_idx, window_max) = relevant_window_items.front().unwrap();
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
