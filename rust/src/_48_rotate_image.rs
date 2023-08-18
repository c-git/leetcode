//! Solution for https://leetcode.com/problems/rotate-image
//! 48. Rotate Image

use std::mem;

struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
    #[must_use]
    fn next(&self, size: usize) -> Self {
        Self {
            row: self.col,
            col: size - 1 - self.row,
        }
    }
}

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        // Used working https://docs.google.com/spreadsheets/d/1fpLqmffz_Z-sxSXOH5w6oB63piPJTRQl0c4sviRZ0hI/
        // to figure out formula to go from one cell to the next (needed the 9xx because 01 is not a valid number on leetcode)
        let n = matrix.len();
        let mut last_pixel_for_row = n - 2;
        for layer in 0..n / 2 {
            for col in layer..=last_pixel_for_row {
                let mut pos = Pos::new(col, layer);
                let mut temp = *Self::get_pos_in_matrix(&pos, matrix);
                for _ in 0..4 {
                    pos = pos.next(n);
                    let next_val = Self::get_pos_in_matrix(&pos, matrix);
                    mem::swap(next_val, &mut temp);
                }
            }
            last_pixel_for_row -= 1;
        }
    }

    fn get_pos_in_matrix<'a>(pos: &Pos, matrix: &'a mut [Vec<i32>]) -> &'a mut i32 {
        matrix.get_mut(pos.row).unwrap().get_mut(pos.col).unwrap()
    }

    pub fn print_matrix(matrix: &[Vec<i32>]) {
        // For debugging
        for row in matrix {
            println!("{row:?}");
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
