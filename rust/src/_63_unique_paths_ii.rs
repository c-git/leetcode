//! Solution for https://leetcode.com/problems/unique-paths-ii
//! 63. Unique Paths II

use std::mem;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        // Based off my recollection of my last solution (probably not similar given the time difference but don't feel like looking it up)
        let n = obstacle_grid[0].len();
        let mut prev_row = vec![0; n];
        let mut curr_row = prev_row.clone();
        prev_row[0] = 1; // Set starting position to reachable 1 way (will get "erased" if it is obstructed in first iteration)

        for row in obstacle_grid.iter() {
            debug_assert_eq!(row.len(), n);

            // Only one way to get to leftmost cell (from above)
            curr_row[0] = if row[0] == 0 { prev_row[0] } else { 0 };

            for (col_idx, cell) in row.iter().enumerate().skip(1) {
                // There are two ways to get to this cell, from above or from the left of it
                debug_assert!(cell == &0 || cell == &1);
                curr_row[col_idx] = if cell == &0 {
                    // Cell unobstructed
                    prev_row[col_idx] + curr_row[col_idx - 1]
                } else {
                    // Cell obstructed we cannot get to here
                    0
                };
            }
            mem::swap(&mut prev_row, &mut curr_row);
        }
        *prev_row.last().expect("Should have at least 1 value")
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![0,0,0],vec![0,1,0],vec![0,0,0]], 2)]
    #[case(vec![vec![0,1],vec![0,0]], 1)]
    fn case(#[case] obstacle_grid: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::unique_paths_with_obstacles(obstacle_grid);
        assert_eq!(actual, expected);
    }
}
