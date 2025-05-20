//! Solution for https://leetcode.com/problems/zero-array-transformation-i
//! 3355. Zero Array Transformation I

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn is_zero_array(nums: Vec<i32>, mut queries: Vec<Vec<i32>>) -> bool {
        // Sort queries so we can process them in order
        queries.sort_unstable();

        // Stores the endpoints for ranges we are currently in
        let mut heap_query_ends = BinaryHeap::new();

        // Stores the number or range we are in meaning number of values we can decrement
        let mut decrement_range = 0;

        // Tracks the next query to begin
        let mut next_query_index = 0;

        for (i, num) in nums.into_iter().enumerate() {
            // Start any applicable ranges
            while next_query_index < queries.len() && queries[next_query_index][0] as usize <= i {
                decrement_range += 1;
                heap_query_ends.push(Reverse(queries[next_query_index][1] as usize));
                next_query_index += 1;
            }

            // Exit any ranges that no longer apply
            while let Some(Reverse(next_end)) = heap_query_ends.peek() {
                if next_end >= &i {
                    // Not ready to be removed yet
                    break;
                }
                decrement_range -= 1;
                heap_query_ends.pop();
            }

            // Confirm this number is able to be decremented to 0
            if num > decrement_range {
                // Not possible to decrement this value to 0
                return false;
            }
        }
        // No violations found in array
        true
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,0,1], vec![vec![0,2]], true)]
    #[case(vec![4,3,2,1], vec![vec![1,3],vec![0,2]], false)]
    fn case(#[case] nums: Vec<i32>, #[case] queries: Vec<Vec<i32>>, #[case] expected: bool) {
        let actual = Solution::is_zero_array(nums, queries);
        assert_eq!(actual, expected);
    }
}
