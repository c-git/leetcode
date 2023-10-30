//! Solution for https://leetcode.com/problems/set-matrix-zeroes
//! 73. Set Matrix Zeroes

use std::collections::BTreeSet;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let row_count = matrix.len();
        let col_count = matrix[0].len();
        let mut rows_to_0: BTreeSet<usize> = BTreeSet::new();
        let mut cols_to_0: BTreeSet<usize> = BTreeSet::new();
        for (idx_row, row_values) in matrix.iter().enumerate() {
            for (idx_col, cell_value) in row_values.iter().enumerate() {
                if *cell_value == 0 {
                    rows_to_0.insert(idx_row);
                    cols_to_0.insert(idx_col);
                }
            }
        }

        for row in rows_to_0 {
            for col in 0..col_count {
                matrix[row][col] = 0;
            }
        }
        for col in cols_to_0 {
            #[allow(clippy::needless_range_loop)]
            for row in 0..row_count {
                matrix[row][col] = 0;
            }
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
    #[case(vec![vec![1,1,1],vec![1,0,1],vec![1,1,1]], vec![vec![1,0,1],vec![0,0,0],vec![1,0,1]])]
    #[case(vec![vec![0,1,2,0],vec![3,4,5,2],vec![1,3,1,5]], vec![vec![0,0,0,0],vec![0,4,5,0],vec![0,3,1,0]])]
    fn case(#[case] mut matrix: Vec<Vec<i32>>, #[case] expected: Vec<Vec<i32>>) {
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, expected);
    }
}
