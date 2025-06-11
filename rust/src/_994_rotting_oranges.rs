//! Solution for https://leetcode.com/problems/rotting-oranges
//! 994. Rotting Oranges

use std::collections::VecDeque;

const EMPTY: i32 = 0;
const FRESH: i32 = 1;
const ROTTEN: i32 = 2;

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let mut fresh_count = 0;
        let mut rotten_queue = VecDeque::new();
        for (row_idx, row) in grid.iter().enumerate() {
            for (col_idx, &cell) in row.iter().enumerate() {
                match cell {
                    EMPTY => {}
                    FRESH => fresh_count += 1,
                    ROTTEN => rotten_queue.push_back((row_idx, col_idx, 0)),
                    _ => unreachable!("by problem constraints"),
                }
            }
        }

        while let Some((row, col, time)) = rotten_queue.pop_front() {
            result = result.max(time);

            // Up
            if row > 0 {
                rot_cell(
                    row - 1,
                    col,
                    time,
                    &mut fresh_count,
                    &mut grid,
                    &mut rotten_queue,
                );
            }

            // Down
            if row < grid.len() - 1 {
                rot_cell(
                    row + 1,
                    col,
                    time,
                    &mut fresh_count,
                    &mut grid,
                    &mut rotten_queue,
                );
            }

            // Left
            if col > 0 {
                rot_cell(
                    row,
                    col - 1,
                    time,
                    &mut fresh_count,
                    &mut grid,
                    &mut rotten_queue,
                );
            }

            // Right
            if col < grid[0].len() - 1 {
                rot_cell(
                    row,
                    col + 1,
                    time,
                    &mut fresh_count,
                    &mut grid,
                    &mut rotten_queue,
                );
            }
        }

        if fresh_count == 0 {
            result
        } else {
            -1
        }
    }
}

#[inline]
fn rot_cell(
    row: usize,
    col: usize,
    time: i32,
    fresh_count: &mut u32,
    grid: &mut [Vec<i32>],
    rotten_queue: &mut VecDeque<(usize, usize, i32)>,
) {
    if grid[row][col] != FRESH {
        return;
    }
    grid[row][col] = ROTTEN;
    *fresh_count -= 1;
    rotten_queue.push_back((row, col, time + 1));
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
