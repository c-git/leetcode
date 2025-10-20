//! Solution for https://leetcode.com/problems/rotting-oranges
//! 994. Rotting Oranges

use std::collections::VecDeque;

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;

        // Find initial rotten oranges
        let mut rotten_queue = VecDeque::new(); // (Row, Col, Time)
        for (row_idx, row) in grid.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                let cell: CellStatus = cell.into();
                if cell.is_rotten() {
                    rotten_queue.push_back((row_idx, col_idx, 0));
                }
            }
        }

        // Each step spread the rot to adjacent cells
        while let Some((row_idx, col_idx, time)) = rotten_queue.pop_front() {
            // Up
            if row_idx > 0 {
                Self::mark_rotten(
                    row_idx - 1,
                    col_idx,
                    time,
                    &mut grid,
                    &mut rotten_queue,
                    &mut result,
                );
            }

            // Down
            if row_idx < grid.len() - 1 {
                Self::mark_rotten(
                    row_idx + 1,
                    col_idx,
                    time,
                    &mut grid,
                    &mut rotten_queue,
                    &mut result,
                );
            }

            // Left
            if col_idx > 0 {
                Self::mark_rotten(
                    row_idx,
                    col_idx - 1,
                    time,
                    &mut grid,
                    &mut rotten_queue,
                    &mut result,
                );
            }

            // Right
            if col_idx < grid[0].len() - 1 {
                Self::mark_rotten(
                    row_idx,
                    col_idx + 1,
                    time,
                    &mut grid,
                    &mut rotten_queue,
                    &mut result,
                );
            }
        }

        // Check if any fresh oranges remain then -1 else result
        if grid.iter().flatten().any(|orange| {
            let cell_status: CellStatus = orange.into();
            cell_status.is_fresh()
        }) {
            -1
        } else {
            result
        }
    }

    fn mark_rotten(
        row_idx: usize,
        col_idx: usize,
        time: i32,
        grid: &mut [Vec<i32>],
        rotten_queue: &mut VecDeque<(usize, usize, i32)>,
        result: &mut i32,
    ) {
        let cell = &mut grid[row_idx][col_idx];
        let cell_status: CellStatus = (&(*cell)).into();
        if !cell_status.is_fresh() {
            // This is not a fresh orange no action needed
            return;
        }
        *cell = CellStatus::Rotten.into();
        let new_time_step = time + 1;
        *result = (*result).max(new_time_step);
        rotten_queue.push_back((row_idx, col_idx, new_time_step));
    }
}

enum CellStatus {
    /// 0 representing an empty cell,
    Empty,
    /// 1 representing a fresh orange, or
    Fresh,
    /// 2 representing a rotten orange
    Rotten,
}

impl CellStatus {
    /// Returns `true` if the status is [`Fresh`].
    ///
    /// [`Fresh`]: Status::Fresh
    #[must_use]
    fn is_fresh(&self) -> bool {
        matches!(self, Self::Fresh)
    }

    /// Returns `true` if the status is [`Rotten`].
    ///
    /// [`Rotten`]: Status::Rotten
    #[must_use]
    fn is_rotten(&self) -> bool {
        matches!(self, Self::Rotten)
    }
}

impl From<&i32> for CellStatus {
    fn from(value: &i32) -> Self {
        match value {
            0 => Self::Empty,
            1 => Self::Fresh,
            2 => Self::Rotten,
            _ => unreachable!("by problem constraint"),
        }
    }
}

impl From<CellStatus> for i32 {
    fn from(value: CellStatus) -> Self {
        match value {
            CellStatus::Empty => 0,
            CellStatus::Fresh => 1,
            CellStatus::Rotten => 2,
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
    #[case(vec![vec![2,1,1],vec![1,1,0],vec![0,1,1]], 4)]
    #[case(vec![vec![2,1,1],vec![0,1,1],vec![1,0,1]], -1)]
    #[case(vec![vec![0,2]], 0)]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::oranges_rotting(grid);
        assert_eq!(actual, expected);
    }
}
