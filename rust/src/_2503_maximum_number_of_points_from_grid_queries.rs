//! Solution for https://leetcode.com/problems/maximum-number-of-points-from-grid-queries
//! 2503. Maximum Number of Points From Grid Queries

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; queries.len()];
        let row_count = grid.len();
        let col_count = grid[0].len();
        let mut max_seen = 0;
        let mut queries: Vec<_> = queries
            .into_iter()
            .enumerate()
            .map(|(i, query)| (query, i))
            .collect();
        queries.sort_unstable();

        let mut seen = vec![vec![false; col_count]; row_count];
        let mut cell_heap = BinaryHeap::new();
        cell_heap.push(Reverse((grid[0][0], 0, 0)));
        for (query, index) in queries.into_iter() {
            while let Some(Reverse((cell_value, ..))) = cell_heap.peek() {
                if query <= *cell_value {
                    break;
                }
                let Reverse((_, row, col)) = cell_heap.pop().expect("just checked to enter loop");
                if seen[row][col] {
                    continue;
                }
                max_seen += 1;
                seen[row][col] = true;
                if row > 0 {
                    Self::visit_cell(row - 1, col, &mut cell_heap, &grid, &seen);
                }
                if col > 0 {
                    Self::visit_cell(row, col - 1, &mut cell_heap, &grid, &seen);
                }
                if row < row_count - 1 {
                    Self::visit_cell(row + 1, col, &mut cell_heap, &grid, &seen);
                }
                if col < col_count - 1 {
                    Self::visit_cell(row, col + 1, &mut cell_heap, &grid, &seen);
                }
            }
            result[index] = max_seen;
        }
        result
    }

    #[inline]
    fn visit_cell(
        row: usize,
        col: usize,
        cell_heap: &mut BinaryHeap<Reverse<(i32, usize, usize)>>,
        grid: &[Vec<i32>],
        seen: &[Vec<bool>],
    ) {
        if !seen[row][col] {
            cell_heap.push(Reverse((grid[row][col], row, col)));
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
    #[case(vec![vec![1,2,3],vec![2,5,7],vec![3,5,1]], vec![5,6,2], vec![5,8,1])]
    #[case(vec![vec![5,2,1],vec![1,1,2]], vec![3], vec![0])]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] queries: Vec<i32>, #[case] expected: Vec<i32>) {
        let actual = Solution::max_points(grid, queries);
        assert_eq!(actual, expected);
    }
}
