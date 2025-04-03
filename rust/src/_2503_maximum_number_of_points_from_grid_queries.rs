//! Solution for https://leetcode.com/problems/maximum-number-of-points-from-grid-queries
//! 2503. Maximum Number of Points From Grid Queries

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; queries.len()];
        let row_count = grid.len();
        let col_count = grid[0].len();
        let mut max_seen = 0;
        let mut query_heap = BinaryHeap::with_capacity(queries.len());
        for (i, query) in queries.into_iter().enumerate() {
            query_heap.push(Reverse((query, i)));
        }

        let mut seen = vec![vec![false; col_count]; row_count];
        let mut cell_heap = BinaryHeap::new();
        cell_heap.push(Reverse((grid[0][0], 0, 0)));
        while let Some(Reverse((query, index))) = query_heap.pop() {
            while let Some(Reverse((cell_value, row, col))) = cell_heap.peek().copied() {
                if query <= cell_value {
                    break;
                }
                cell_heap.pop(); // Remove from heap not just peek
                if seen[row][col] {
                    continue;
                }
                max_seen += 1;
                seen[row][col] = true;
                let directions: [Option<(usize, usize)>; 4] = [
                    if row > 0 { Some((row - 1, col)) } else { None },
                    if col > 0 { Some((row, col - 1)) } else { None },
                    if row < row_count - 1 {
                        Some((row + 1, col))
                    } else {
                        None
                    },
                    if col < col_count - 1 {
                        Some((row, col + 1))
                    } else {
                        None
                    },
                ];
                for direction in directions {
                    let Some((next_row, next_col)) = direction else {
                        continue;
                    };
                    if !seen[next_row][next_col] {
                        cell_heap.push(Reverse((grid[next_row][next_col], next_row, next_col)));
                    }
                }
            }
            result[index] = max_seen;
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
    #[case(vec![vec![1,2,3],vec![2,5,7],vec![3,5,1]], vec![5,6,2], vec![5,8,1])]
    #[case(vec![vec![5,2,1],vec![1,1,2]], vec![3], vec![0])]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] queries: Vec<i32>, #[case] expected: Vec<i32>) {
        let actual = Solution::max_points(grid, queries);
        assert_eq!(actual, expected);
    }
}
