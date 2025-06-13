//! Solution for https://leetcode.com/problems/min-cost-to-connect-all-points
//! 1584. Min Cost to Connect All Points

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

impl Solution {
    /// Using Prim's Algorithm based on https://www.youtube.com/watch?v=f7JOBJIC-NA
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;

        let mut added = HashSet::new();

        // Distance, node index
        let mut nearest_heap: BinaryHeap<(Reverse<u32>, usize)> = BinaryHeap::new();
        nearest_heap.push((Reverse(0), 0)); // Start at node 0

        while added.len() < points.len() {
            let (Reverse(distance), curr_point) = nearest_heap
                .pop()
                .expect("if not connected this should not be empty");
            if !added.insert(curr_point) {
                // Was already added so skip
                continue;
            }
            result += distance;
            for (other_idx, other_point) in points.iter().enumerate() {
                if !added.contains(&other_idx) {
                    // Not added yet, add edge as candidate
                    let edge_distance = points[curr_point][0].abs_diff(other_point[0])
                        + points[curr_point][1].abs_diff(other_point[1]);
                    nearest_heap.push((Reverse(edge_distance), other_idx));
                }
            }
        }

        result as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![0,0],vec![2,2],vec![3,10],vec![5,2],vec![7,0]], 20)]
    #[case(vec![vec![3,12],vec![-2,5],vec![-4,1]], 18)]
    fn case(#[case] points: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::min_cost_connect_points(points);
        assert_eq!(actual, expected);
    }
}
