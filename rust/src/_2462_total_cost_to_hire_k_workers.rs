//! Solution for https://leetcode.com/problems/total-cost-to-hire-k-workers
//! 2462. Total Cost to Hire K Workers

use std::{cmp::Reverse, collections::BinaryHeap, iter::Rev};

impl Solution {
    pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        let mut result = 0;
        let k = k as usize;
        let candidates = candidates as usize;
        let n = costs.len();

        // Next index to take from the left side
        let mut left = candidates;

        // Next index used on the right side
        let mut right = (n.checked_sub(1 + candidates).unwrap_or_default())
            .max(left) // Use same next as left if bigger
            .min(n - 1); // Max value is last position

        let mut candidate_pool_left = BinaryHeap::with_capacity(left);
        let mut candidate_pool_right = BinaryHeap::with_capacity(n - right);

        // Add candidates from the left
        costs
            .iter()
            .take(left)
            .for_each(|&x| candidate_pool_left.push(Reverse(x)));

        // Add candidates from the right
        costs
            .iter()
            .skip(right + 1)
            .for_each(|&x| candidate_pool_right.push(Reverse(x)));

        for _ in 0..k {
            let left_opt = candidate_pool_left.peek();
            let right_opt = candidate_pool_right.peek();
            let cost = match left_opt.cmp(&right_opt) {
                std::cmp::Ordering::Greater | std::cmp::Ordering::Equal if left_opt.is_some() => {
                    let Reverse(val) = candidate_pool_left.pop().unwrap();
                    if left <= right {
                        candidate_pool_left.push(Reverse(costs[left]));
                        left += 1;
                    }
                    val
                }
                std::cmp::Ordering::Less if right_opt.is_some() => {
                    let Reverse(val) = candidate_pool_right.pop().unwrap();
                    if left <= right {
                        candidate_pool_right.push(Reverse(costs[right]));
                        right -= 1;
                    }
                    val
                }
                _ => unreachable!(
                    "Never expect execution to get here. This would mean both ran empty, left_opt {left_opt:?} right_opt {right_opt:?}"
                ),
            };
            result += cost as i64;
        }
        result
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![17,12,10,2,7,2,11,20,8], 3, 4, 11)]
    #[case(vec![1,2,4,1], 3, 3, 4)]
    #[case(vec![1,2,4,1], 4, 3, 8)]
    #[case(vec![1,2,4,1], 3, 4, 4)]
    #[case(vec![1,2,4,1], 4, 4, 8)]
    #[case(vec![31,25,72,79,74,65,84,91,18,59,27,9,81,33,17,58], 11, 2, 423)]
    fn case(
        #[case] costs: Vec<i32>,
        #[case] k: i32,
        #[case] candidates: i32,
        #[case] expected: i64,
    ) {
        let actual = Solution::total_cost(costs, k, candidates);
        assert_eq!(actual, expected);
    }
}
