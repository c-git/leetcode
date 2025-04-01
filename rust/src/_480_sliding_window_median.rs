//! Solution for https://leetcode.com/problems/sliding-window-median
//! 480. Sliding Window Median

use std::cmp::Reverse;
use std::collections::BinaryHeap;

type LeftHeap = BinaryHeap<i32>;
type RightHeap = BinaryHeap<Reverse<i32>>;

impl Solution {
    /// After hints
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        let k = k as usize;
        let mut result = Vec::with_capacity(nums.len() - k + 1);

        // Establish preconditions for loop
        let mut left_heap = BinaryHeap::new();
        let mut right_heap = BinaryHeap::new();
        for x in nums.iter().copied().take(k) {
            Self::insert_value(x, &mut left_heap, &mut right_heap)
        }
        result.push(Self::median_of(&left_heap, &right_heap));

        // Loop over rest of list
        let mut pop_value = nums[0];
        for window in nums.windows(k).skip(1) {
            Self::remove_value(pop_value, &mut left_heap, &mut right_heap);
            pop_value = window[0];
            Self::insert_value(*window.last().unwrap(), &mut left_heap, &mut right_heap);
            result.push(Self::median_of(&left_heap, &right_heap));
        }

        result
    }

    fn remove_value(pop_value: i32, left_heap: &mut LeftHeap, right_heap: &mut RightHeap) {
        let mut is_removed = false;

        if !left_heap.is_empty() && pop_value <= *left_heap.peek().unwrap() {
            left_heap.retain(|x| {
                if !is_removed && x == &pop_value {
                    is_removed = true;
                    false
                } else {
                    true
                }
            });
        } else {
            right_heap.retain(|x| {
                if !is_removed && x == &Reverse(pop_value) {
                    is_removed = true;
                    false
                } else {
                    true
                }
            });
        }
        Self::balance_heaps(left_heap, right_heap);
    }

    fn balance_heaps(left_heap: &mut LeftHeap, right_heap: &mut RightHeap) {
        while left_heap.len() < right_heap.len() || left_heap.len().abs_diff(right_heap.len()) > 1 {
            if left_heap.len() < right_heap.len() {
                let Reverse(right_min_value) = right_heap.pop().unwrap();
                left_heap.push(right_min_value);
            } else {
                let left_max_value = left_heap.pop().unwrap();
                right_heap.push(Reverse(left_max_value));
            }
        }
    }

    fn insert_value(new_value: i32, left_heap: &mut LeftHeap, right_heap: &mut RightHeap) {
        if let Some(left_max) = left_heap.peek() {
            if new_value <= *left_max {
                left_heap.push(new_value);
            } else {
                right_heap.push(Reverse(new_value));
            }
        } else {
            left_heap.push(new_value);
        }

        Self::balance_heaps(left_heap, right_heap);
    }

    fn median_of(left_heap: &LeftHeap, right_heap: &RightHeap) -> f64 {
        if left_heap.len() > right_heap.len() {
            *left_heap.peek().unwrap() as f64
        } else {
            debug_assert_eq!(left_heap.len(), right_heap.len());
            (*left_heap.peek().unwrap() as f64 + right_heap.peek().unwrap().0 as f64) / 2.0
        }
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,3,-1,-3,5,3,6,7], 3, vec![1.00000,-1.00000,-1.00000,3.00000,5.00000,6.00000])]
    #[case(vec![1,2,3,4,2,3,1,4,2], 3, vec![2.00000,3.00000,3.00000,3.00000,2.00000,3.00000,2.00000])]
    #[case(vec![1,3,-1,-3,5,3,6,7], 8, vec![3.0])]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: Vec<f64>) {
        let actual = Solution::median_sliding_window(nums, k);
        assert_eq!(actual, expected);
    }
}
