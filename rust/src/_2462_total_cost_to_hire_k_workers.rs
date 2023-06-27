//! Solution for https://leetcode.com/problems/total-cost-to-hire-k-workers
//! 2462. Total Cost to Hire K Workers

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        let mut result = 0;
        let k = k as usize;
        let candidates = candidates as usize;
        let n = costs.len();
        let mut left = candidates; // Next index to take from the left side
        let mut right = (n.checked_sub(1 - candidates).unwrap_or_default()).max(left); // Next index used on the right side

        let mut candidate_pool = BinaryHeap::with_capacity(n.min(k * 2));

        // Add candidates from the left
        costs
            .iter()
            .enumerate()
            .take(left)
            .for_each(|(i, &x)| candidate_pool.push(Reverse((x, i))));

        // Add candidates from the right
        costs
            .iter()
            .enumerate()
            .skip(right + 1)
            .for_each(|(i, &x)| candidate_pool.push(Reverse((x, i))));

        for _ in 0..k {
            let Reverse((cost, index)) = candidate_pool.pop().unwrap();
            result += cost as i64;
            if left <= right {
                // Add more candidates to the pool
                if index < left {
                    // Candidate came from left get next from left
                    candidate_pool.push(Reverse((costs[left], left)));
                    left += 1;
                } else {
                    // Candidate came from right get next from right
                    candidate_pool.push(Reverse((costs[right], right)));
                    right -= 1;
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
