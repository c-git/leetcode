//! Solution for https://leetcode.com/problems/number-of-islands
//! 200. Number of Islands

impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut result = 0;
        for row_idx in 0..grid.len() {
            for col_idx in 0..grid[0].len() {
                if grid[row_idx][col_idx] == '1' {
                    explore_island(&mut grid, row_idx, col_idx);
                    result += 1;
                }
            }
        }
        result
    }
}

fn explore_island(grid: &mut [Vec<char>], row_idx: usize, col_idx: usize) {
    let mut stack = vec![];
    debug_assert_eq!(grid[row_idx][col_idx], '1');
    grid[row_idx][col_idx] = '0';
    stack.push((row_idx, col_idx));
    while let Some((row, col)) = stack.pop() {
        // Up
        if row > 0 {
            clear_and_add_to_stack(grid, &mut stack, row - 1, col);
        }
        // Down
        if row < grid.len() - 1 {
            clear_and_add_to_stack(grid, &mut stack, row + 1, col);
        }
        // Left
        if col > 0 {
            clear_and_add_to_stack(grid, &mut stack, row, col - 1);
        }
        // Right
        if col < grid[0].len() - 1 {
            clear_and_add_to_stack(grid, &mut stack, row, col + 1);
        }
    }
}

fn clear_and_add_to_stack(
    grid: &mut [Vec<char>],
    stack: &mut Vec<(usize, usize)>,
    row: usize,
    col: usize,
) {
    if grid[row][col] == '1' {
        grid[row][col] = '0';
        stack.push((row, col));
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec!['1','1','1','1','0'],vec!['1','1','0','1','0'],vec!['1','1','0','0','0'],vec!['0','0','0','0','0']], 1)]
    #[case(vec![vec!['1','1','0','0','0'],vec!['1','1','0','0','0'],vec!['0','0','1','0','0'],vec!['0','0','0','1','1']], 3)]
    fn case(#[case] grid: Vec<Vec<char>>, #[case] expected: i32) {
        let actual = Solution::num_islands(grid);
        assert_eq!(actual, expected);
    }
}
