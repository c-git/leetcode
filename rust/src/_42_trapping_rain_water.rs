//! Solution for https://leetcode.com/problems/trapping-rain-water
//! 42. Trapping Rain Water

use std::{cmp::min, collections::VecDeque};

impl Solution {
    pub fn trap(heights: Vec<i32>) -> i32 {
        let n = heights.len();

        // Peaks coming from the left (Height, Index)
        let mut peaks_left = VecDeque::new();
        peaks_left.push_back((heights[0], 0));
        for (idx_left, height) in heights.iter().enumerate().skip(1) {
            if height >= &peaks_left.back().unwrap().0 {
                // This elevation is equal or higher
                peaks_left.push_back((*height, idx_left));
            }
        }

        // Peaks coming from the right (Height, Index)
        let mut peaks_right = VecDeque::new();
        peaks_right.push_front((heights[n - 1], n - 1));
        for (idx_right, height) in heights.iter().enumerate().rev().skip(1) {
            if height >= &peaks_right.front().unwrap().0 {
                peaks_right.push_front((*height, idx_right));
            }
        }

        // Join lists of peaks (Max of left will be at the back of deque, Max of right will be at front of deque)
        while !peaks_right.is_empty()
            && peaks_left.back().unwrap().1 >= peaks_right.front().unwrap().1
        {
            peaks_right.pop_front(); // Remove overlap area
        }

        peaks_left.append(&mut peaks_right);
        let peaks = peaks_left; // Now contains all relevant peaks

        if peaks.len() < 2 {
            // Not enough peaks to catch water (Don't see how this could happen though)
            return 0;
        }

        let mut result = 0;

        // Calculate trapped water
        let mut next_idx_peak = 2;
        let mut catch_height = min(peaks[0].0, peaks[1].0);
        let mut covered_height_idx = peaks[1].1;
        for (i, height) in heights.iter().enumerate() {
            if covered_height_idx < i {
                debug_assert!(next_idx_peak < peaks.len(), "All indices should be covered thus if this one is not then there must be more peaks");
                catch_height = min(peaks[next_idx_peak - 1].0, peaks[next_idx_peak].0);
                covered_height_idx = peaks[next_idx_peak].1;
                next_idx_peak += 1
            }
            if catch_height > *height {
                result += catch_height - height;
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
    #[case(vec![0,1,0,2,1,0,1,3,2,1,2,1], 6)]
    #[case(vec![4,2,0,3,2,5], 9)]
    #[case(vec![8,0,1,1,5,3,4,0,1,2], 17)]
    #[case(vec![20,0,1,1,5,3,4,0,1,2], 17)]
    #[case(vec![1,2,3,4,5,20,0,1,1,5,3,4,0,1,2], 17)]
    #[case(vec![1,2,3,4,5], 0)]
    #[case(vec![5,4,3,2,1,0], 0)]
    #[case(vec![5,5,1,7,1,1,5,2,7,6], 23)]
    fn case(#[case] height: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::trap(height);
        assert_eq!(actual, expected);
    }
}
