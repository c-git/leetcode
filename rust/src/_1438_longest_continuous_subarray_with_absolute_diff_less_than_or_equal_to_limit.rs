//! Solution for https://leetcode.com/problems/longest-continuous-subarray-with-absolute-diff-less-than-or-equal-to-limit
//! 1438. Longest Continuous Subarray With Absolute Diff Less Than or Equal to Limit

use std::{cmp::Reverse, collections::VecDeque, fmt::Debug};

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let mut result = 0;
        let mut min_queue: MonotonicQueue<Reverse<i32>> = Default::default();
        let mut max_queue: MonotonicQueue<i32> = Default::default();
        for (index, num) in nums.into_iter().enumerate() {
            min_queue.append(QueueElement {
                value: Reverse(num),
                index,
            });
            max_queue.append(QueueElement { value: num, index });
            while Self::diff(&min_queue, &max_queue) > limit {
                // Last should always be the current so that diff should be 0 so this should never empty either queue
                Self::pop_older(&mut min_queue, &mut max_queue);
            }
            let index_of_min = min_queue.peek_front().unwrap().index;
            let index_of_max = max_queue.peek_front().unwrap().index;
            let length = index - index_of_max.min(index_of_min) + 1;
            result = result.max(length);
        }
        result as i32
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

impl<T: Debug> Debug for MonotonicQueue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut queue_values = String::new();
        for element in self.queue.iter() {
            queue_values.push_str(&format!("({}, {:?}), ", element.index, element.value));
        }
        write!(f, "{queue_values}")
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
    fn case(#[case] nums: Vec<i32>, #[case] limit: i32, #[case] expected: i32) {
        let actual = Solution::longest_subarray(nums, limit);
        assert_eq!(actual, expected);
    }
}
