//! Solution for https://leetcode.com/problems/rotting-oranges
//! 994. Rotting Oranges

use std::collections::VecDeque;

const FRESH: i32 = 1;
const ROTTEN: i32 = 2;

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut max_seconds = 0;

        // Collect the starting rotten oranges
        let mut rotten_list = VecDeque::new();
        for (row_idx, row) in grid.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                if cell == &ROTTEN {
                    rotten_list.push_back((row_idx, col_idx, 0));
                }
            }
        }

        // Use BFS to change adjacent oranges to rotten
        while let Some((row, col, seconds)) = rotten_list.pop_front() {
            if row > 0 {
                check_cell(
                    row - 1,
                    col,
                    seconds,
                    &mut grid,
                    &mut rotten_list,
                    &mut max_seconds,
                );
            }
            if col > 0 {
                check_cell(
                    row,
                    col - 1,
                    seconds,
                    &mut grid,
                    &mut rotten_list,
                    &mut max_seconds,
                );
            }
            if row < grid.len() - 1 {
                check_cell(
                    row + 1,
                    col,
                    seconds,
                    &mut grid,
                    &mut rotten_list,
                    &mut max_seconds,
                );
            }
            if col < grid[0].len() - 1 {
                check_cell(
                    row,
                    col + 1,
                    seconds,
                    &mut grid,
                    &mut rotten_list,
                    &mut max_seconds,
                );
            }
        }

        // Check for any remaining fresh oranges
        for row in grid.iter() {
            for cell in row {
                if cell == &FRESH {
                    // Found an orange this is still fresh
                    return -1;
                }
            }
        }
        max_seconds
    }
}

fn check_cell(
    row: usize,
    col: usize,
    seconds: i32,
    grid: &mut [Vec<i32>],
    rotten_list: &mut VecDeque<(usize, usize, i32)>,
    max_seconds: &mut i32,
) {
    if grid[row][col] != FRESH {
        return;
    }
    grid[row][col] = ROTTEN;
    rotten_list.push_back((row, col, seconds + 1));
    *max_seconds = (*max_seconds).max(seconds + 1);
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
