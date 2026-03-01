//! Solution for https://leetcode.com/problems/longest-continuous-subarray-with-absolute-diff-less-than-or-equal-to-limit
//! 1438. Longest Continuous Subarray With Absolute Diff Less Than or Equal to Limit

use std::{cmp::Reverse, collections::VecDeque, fmt::Debug};

impl Solution {
    /// Comparing my not working solution with https://www.youtube.com/watch?v=V-ecDfY5xEw
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let mut my_solution = SolutionState::default();
        for new_right in 0..nums.len() {
            my_solution = Self::my_solution(new_right, my_solution, &nums, limit);
        }
        my_solution.result as i32
    }

    fn my_solution(
        new_right: usize,
        solution_state: SolutionState<i32>,
        nums: &[i32],
        limit: i32,
    ) -> SolutionState<i32> {
        let SolutionState {
            mut left,
            min_queue,
            max_queue,
            mut result,
        } = solution_state;
        let num = nums[new_right];
        let mut min_queue: MonotonicQueue<Reverse<i32>> = min_queue.into();
        let mut max_queue: MonotonicQueue<i32> = max_queue.into();
        min_queue.append(QueueElement {
            value: Reverse(num),
            index: new_right,
        });
        max_queue.append(QueueElement {
            value: num,
            index: new_right,
        });
        while Self::diff(&min_queue, &max_queue) > limit {
            // Last should always be the current so that diff should be 0 so this should never empty either queue
            Self::pop_older(&mut min_queue, &mut max_queue);
            let index_of_min = min_queue.peek_front().unwrap().index;
            let index_of_max = max_queue.peek_front().unwrap().index;
            left = index_of_max.min(index_of_min);
        }
        let length = new_right - left + 1;
        result = result.max(length);
        SolutionState {
            left,
            min_queue: min_queue.queue,
            max_queue: max_queue.queue,
            result,
        }
    }

    /// Assumes input queues are not empty
    fn diff(min_queue: &MonotonicQueue<Reverse<i32>>, max_queue: &MonotonicQueue<i32>) -> i32 {
        let result =
            max_queue.peek_front().unwrap().value - min_queue.peek_front().unwrap().value.0;
        debug_assert!(result >= 0);
        result
    }

    fn pop_older(
        min_queue: &mut MonotonicQueue<Reverse<i32>>,
        max_queue: &mut MonotonicQueue<i32>,
    ) {
        let min_index = min_queue.peek_front().unwrap().index;
        let max_index = max_queue.peek_front().unwrap().index;
        if min_index < max_index {
            min_queue.pop_front();
        } else {
            max_queue.pop_front();
        }
    }
}

#[derive(Debug, Default)]
struct SolutionState<T> {
    left: usize,
    min_queue: VecDeque<QueueElement<Reverse<T>>>,
    max_queue: VecDeque<QueueElement<T>>,
    result: usize,
}

#[derive(Default)]
/// Implements a Max queue where the largest element is always at the front
struct MonotonicQueue<T> {
    queue: VecDeque<QueueElement<T>>,
}

struct QueueElement<T> {
    value: T,
    index: usize,
}

impl<T> MonotonicQueue<T> {
    /// Value is added to back (unless equal to last) and pushes out any values that are smaller
    fn append(&mut self, new_element: QueueElement<T>)
    where
        T: Ord + Debug,
    {
        while let Some(last) = self.queue.back() {
            if last.value < new_element.value {
                // Remove any items that are smaller to maintain monotonicity
                self.queue.pop_back();
            } else {
                break;
            }
        }
        if let Some(last) = self.queue.back()
            && last.value == new_element.value
        {
            // Don't add keep oldest equal value
            return;
        }

        self.queue.push_back(new_element);
    }

    fn peek_front(&self) -> Option<&QueueElement<T>> {
        self.queue.front()
    }

    fn pop_front(&mut self) -> Option<QueueElement<T>> {
        self.queue.pop_front()
    }
}

impl<T> From<VecDeque<QueueElement<T>>> for MonotonicQueue<T> {
    fn from(value: VecDeque<QueueElement<T>>) -> Self {
        Self { queue: value }
    }
}

impl<T: Debug> Debug for QueueElement<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {:?})", self.index, self.value)
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![8,2,4,7], 4, 2)]
    #[case(vec![10,1,2,4,7,2], 5, 4)]
    #[case(vec![4,2,2,2,4,4,2,2], 0, 3)]
    #[case(vec![4,10,2,6,1], 10, 5)]
    #[case(vec![24,12,71,33,5,87,10,11,3,58,2,97,97,36,32,35,15,80,24,45,38,9,22,21,33,68,22,85,35,83,92,38,59,90,42,64,61,15,4,40,50,44,54,25,34,14,33,94,66,27,78,56,3,29,3,51,19,5,93,21,58,91,65,87,55,70,29,81,89,67,58,29,68,84,4,51,87,74,42,85,81,55,8,95,39], 87, 25)]
    fn case(#[case] nums: Vec<i32>, #[case] limit: i32, #[case] expected: i32) {
        let actual = Solution::longest_subarray(nums, limit);
        assert_eq!(actual, expected);
    }
}
