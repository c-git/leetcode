//! Solution for https://leetcode.com/problems/set-matrix-zeroes
//! 73. Set Matrix Zeroes

#[allow(clippy::needless_range_loop)]
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        // Source: https://www.youtube.com/watch?v=T41rL0L3Pnw
        // Constant space solution
        // Solution idea.
        //      - Use first row as indicator for columns to 0
        //      - Use first column (less top cell) as indicator for rows to 0
        //      - Track bool to see if first row should be zeroed

        let should_0_first_row = matrix[0].iter().any(|&x| x == 0);

        let row_count = matrix.len();
        let col_count = matrix[0].len();

        for row in 0..row_count {
            for col in 0..col_count {
                if matrix[row][col] == 0 {
                    matrix[row][0] = 0; // Set row to be zeroed
                    matrix[0][col] = 0; // Set col to be zeroed
                }
            }
        }

        for row in 1..row_count {
            if matrix[row][0] == 0 {
                // row marked to be zeroed, set all values to 0
                matrix[row].iter_mut().for_each(|x| *x = 0);
            }
        }
        for col in 0..col_count {
            if matrix[0][col] == 0 {
                // col marked to be zeroed, set all values to 0
                matrix.iter_mut().for_each(|row_values| row_values[col] = 0);
            }
        }
        if should_0_first_row {
            matrix[0].iter_mut().for_each(|x| *x = 0);
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
    fn case(#[case] mut matrix: Vec<Vec<i32>>, #[case] expected: Vec<Vec<i32>>) {
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, expected);
    }
}
