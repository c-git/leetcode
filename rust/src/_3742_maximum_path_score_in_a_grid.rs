//! Solution for https://leetcode.com/problems/maximum-path-score-in-a-grid
//! 3742. Maximum Path Score in a Grid

impl Solution {
    /// Use Brute force as k is limited to 1000
    pub fn max_path_score(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let row_count = grid.len();
        let col_count = grid[0].len();
        let k = k as u16;
        let mut result = -1;

        let mut stack: Vec<(usize, usize, i32, u16)> = Default::default();
        stack.push((0, 0, 0, 0));
        while let Some((row, col, score, cost)) = stack.pop() {
            let cost = cost + 1.min(grid[row][col] as u16);
            if cost > k {
                continue;
            }
            let score = score + grid[row][col];
            if row == row_count - 1 && col == col_count - 1 {
                result = result.max(score);
                continue;
            }
            if row < row_count - 1 {
                stack.push((row + 1, col, score, cost));
            }
            if col < col_count - 1 {
                stack.push((row, col + 1, score, cost));
            }
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
    #[case(vec![vec![0, 1],vec![2, 0]], 1, 2)]
    #[case(vec![vec![0, 1],vec![1, 2]], 1, -1)]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::max_path_score(grid, k);
        assert_eq!(actual, expected);
    }
}
