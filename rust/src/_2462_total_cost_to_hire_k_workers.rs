//! Solution for https://leetcode.com/problems/total-cost-to-hire-k-workers
//! 2462. Total Cost to Hire K Workers

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

impl Solution {
    pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        // Using VecDeque idea from sak96
        let mut result = 0;
        let candidates = candidates as usize;
        let n = costs.len();
        let mut costs = VecDeque::from(costs);

        let mut candidate_pool = BinaryHeap::with_capacity(n.min(candidates * 2));
        for _ in 0..candidates {
            if let Some(cost) = costs.pop_front() {
                candidate_pool.push(Reverse((cost, false)))
            }
            if let Some(cost) = costs.pop_back() {
                candidate_pool.push(Reverse((cost, true)))
            }
        }

        for _ in 0..k {
            let Reverse((cost, is_right)) = candidate_pool.pop().unwrap();
            result += cost as i64;
            if !costs.is_empty() {
                if is_right {
                    candidate_pool.push(Reverse((costs.pop_back().unwrap(), true)));
                } else {
                    candidate_pool.push(Reverse((costs.pop_front().unwrap(), false)));
                }
            }
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
    #[case(vec![10,1,11,10], 2, 1, 11)]
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
