//! Solution for https://leetcode.com/problems/01-matrix
//! 542. 01 Matrix

use std::collections::VecDeque;

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let count_rows = mat.len();
        let count_cols = mat[0].len();
        let mut result = vec![vec![None; count_cols]; count_rows];

        let mut queue = VecDeque::new();

        // Find 0's
        for (idx_row, row) in mat.iter().enumerate() {
            for (idx_col, cell) in row.iter().enumerate() {
                if *cell == 0 {
                    result[idx_row][idx_col] = Some(0);
                    queue.push_back((idx_row, idx_col));
                }
            }
        }

        while let Some((idx_row, idx_col)) = queue.pop_front() {
            let new_steps = result[idx_row][idx_col].unwrap() + 1;
            for (neighbour_row, neighbour_col) in
                Self::neighbours(idx_row, idx_col, count_rows, count_cols)
            {
                if result[neighbour_row][neighbour_col].is_none() {
                    result[neighbour_row][neighbour_col] = Some(new_steps);
                    queue.push_back((neighbour_row, neighbour_col));
                }
            }
        }

        result
            .into_iter()
            .map(|row| row.into_iter().map(|cell| cell.unwrap()).collect())
            .collect()
    }

    fn neighbours(
        idx_row: usize,
        idx_col: usize,
        count_rows: usize,
        count_cols: usize,
    ) -> Vec<(usize, usize)> {
        let mut result = vec![];
        if idx_row > 0 {
            result.push((idx_row - 1, idx_col));
        }
        if idx_col > 0 {
            result.push((idx_row, idx_col - 1));
        }
        if idx_row < count_rows - 1 {
            result.push((idx_row + 1, idx_col));
        }
        if idx_col < count_cols - 1 {
            result.push((idx_row, idx_col + 1));
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
    #[case(vec![vec![0,0,0],vec![0,1,0],vec![0,0,0]], vec![vec![0,0,0],vec![0,1,0],vec![0,0,0]])]
    #[case(vec![vec![0,0,0],vec![0,1,0],vec![1,1,1]], vec![vec![0,0,0],vec![0,1,0],vec![1,2,1]])]
    fn case(#[case] mat: Vec<Vec<i32>>, #[case] expected: Vec<Vec<i32>>) {
        let actual = Solution::update_matrix(mat);
        assert_eq!(actual, expected);
    }
}
