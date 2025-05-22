//! Solution for https://leetcode.com/problems/zero-array-transformation-iii
//! 3362. Zero Array Transformation III

use std::collections::BinaryHeap;

impl Solution {
    /// Based on  from editorial
    pub fn max_removal(nums: Vec<i32>, mut queries: Vec<Vec<i32>>) -> i32 {
        queries.sort_unstable();
        let mut heap = BinaryHeap::new();
        let mut delta_array = vec![0; nums.len() + 1];
        let mut operations = 0;
        let mut next_query_idx = 0;

        for num_idx in 0..nums.len() {
            operations += delta_array[num_idx];

            // move in queries that match our current position
            while next_query_idx < queries.len() && queries[next_query_idx][0] == num_idx as i32 {
                heap.push(queries[next_query_idx][1] as usize);
                next_query_idx += 1;
            }

            // Pull queries from heap as needed
            while operations < nums[num_idx] && !heap.is_empty() && *heap.peek().unwrap() >= num_idx
            {
                operations += 1;
                let end = heap.pop().unwrap();
                delta_array[end + 1] -= 1;
            }

            // Check if we met the required number of operations or early exit
            if operations < nums[num_idx] {
                return -1;
            }
        }

        // Queries left on the heap were not used
        heap.len() as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![2,0,2], vec![vec![0,2],vec![0,2],vec![1,1]], 1)]
    #[case(vec![1,1,1,1], vec![vec![1,3],vec![0,2],vec![1,3],vec![1,2]], 2)]
    #[case(vec![1,2,3,4], vec![vec![0,3]], -1)]
    fn case(#[case] nums: Vec<i32>, #[case] queries: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::max_removal(nums, queries);
        assert_eq!(actual, expected);
    }
}
