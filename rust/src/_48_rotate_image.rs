//! Solution for https://leetcode.com/problems/rotate-image
//! 48. Rotate Image

impl Solution {
    pub fn rotate(matrix: &mut [Vec<i32>]) {
        let n = matrix.len();
        for row in 0..n / 2 {
            for col in row..n - 1 - row {
                let mut temp = matrix[row][col];
                println!("row: {row}, col: {col}");
                let mut curr_row = row;
                let mut curr_col = col;
                for _ in 1..=4 {
                    (curr_row, curr_col) = (curr_col, n - 1 - curr_row);
                    println!("{curr_row},{curr_col} = {temp}");
                    std::mem::swap(&mut temp, &mut matrix[curr_row][curr_col]);
                }
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
    #[case(vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]], vec![vec![7,4,1],vec![8,5,2],vec![9,6,3]])]
    #[case(vec![vec![5,1,9,11],vec![2,4,8,10],vec![13,3,6,7],vec![15,14,12,16]], vec![vec![15,13,2,5],vec![14,3,4,1],vec![12,6,8,9],vec![16,7,10,11]])]
    #[case(vec![vec![900,901,902,903,904],vec![910,911,912,913,914],vec![920,921,922,923,924],vec![930,931,932,933,934],vec![940,941,942,943,944]],
           vec![vec![940,930,920,910,900],vec![941,931,921,911,901],vec![942,932,922,912,902],vec![943,933,923,913,903],vec![944,934,924,914,904]])]
    #[case(vec![vec![900,901,902,903,904,905],vec![910,911,912,913,914,915],vec![920,921,922,923,924,925],vec![930,931,932,933,934,935],vec![940,941,942,943,944,945],vec![950,951,952,953,954,955]],
           vec![vec![950,940,930,920,910,900],vec![951,941,931,921,911,901],vec![952,942,932,922,912,902],vec![953,943,933,923,913,903],vec![954,944,934,924,914,904],vec![955,945,935,925,915,905]])]
    fn case(#[case] mut matrix: Vec<Vec<i32>>, #[case] expected: Vec<Vec<i32>>) {
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, expected);
    }
}
