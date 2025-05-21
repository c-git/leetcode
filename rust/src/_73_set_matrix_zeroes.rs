//! Solution for https://leetcode.com/problems/set-matrix-zeroes
//! 73. Set Matrix Zeroes

impl Solution {
    pub fn set_zeroes(matrix: &mut [Vec<i32>]) {
        let row_count = matrix.len();
        let col_count = matrix[0].len();
        let mut rows_to_set = vec![false; row_count];
        let mut cols_to_set = vec![false; col_count];
        for (row_idx, row) in matrix.iter().enumerate() {
            for (col_idx, &cell) in row.iter().enumerate() {
                if cell == 0 {
                    rows_to_set[row_idx] = true;
                    cols_to_set[col_idx] = true;
                }
            }
        }
        for (row, _) in rows_to_set.iter().enumerate().filter(|(_, val)| **val) {
            for col in 0..col_count {
                matrix[row][col] = 0;
            }
        }
        for (col, _) in cols_to_set.iter().enumerate().filter(|(_, val)| **val) {
            for row in matrix.iter_mut() {
                row[col] = 0;
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
    #[case(vec![vec![1],vec![0]], vec![vec![0],vec![0]])]
    #[case(vec![vec![-4,-2147483648,6,-7,0],vec![-8,6,-8,-6,0],vec![2147483647,2,-9,-6,-10]], vec![vec![0,0,0,0,0],vec![0,0,0,0,0],vec![2147483647,2,-9,-6,0]])]
    fn case(#[case] mut matrix: Vec<Vec<i32>>, #[case] expected: Vec<Vec<i32>>) {
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, expected);
    }
}
