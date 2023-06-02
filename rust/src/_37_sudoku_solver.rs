use std::collections::BTreeSet;

impl Solution {
    fn solve_sudoku_helper(board: &mut Vec<Vec<char>>, row: usize, col: usize) -> bool {
        debug_assert_eq!(board[row][col], '.');
        let candidates = Self::get_candidates(board, row, col);
        if candidates.is_empty() {
            return false; // No candidates we cannot continue down this path
        }

        // Fill current spot so we are able to detect that it is not available when we search
        board[row][col] = *candidates.iter().next().unwrap();

        let (next_unfilled_row, next_unfilled_col) =
            if let Some(unfilled) = Self::get_next_unfilled_cell(board, row, col) {
                unfilled
            } else {
                debug_assert_eq!(candidates.len(), 1);
                return true; // Filled in last value
            };

        for candidate in candidates {
            // Assumption: One of these values must work
            board[row][col] = candidate;
            if Self::solve_sudoku_helper(board, next_unfilled_row, next_unfilled_col) {
                return true;
            }
        }

        board[row][col] = '.'; // Reverse attempt tried
        false
    }

    fn get_next_unfilled_cell(
        board: &[Vec<char>],
        start_row: usize,
        start_col: usize,
    ) -> Option<(usize, usize)> {
        // Check the rest of the current line
        if let Some(col) = board[start_row][start_col..]
            .iter()
            .enumerate()
            .find_map(|(i, &x)| if x != '.' { None } else { Some(i) })
        {
            return Some((start_row, start_col + col));
        }

        // Check the other lines
        for (row, row_values) in board.iter().enumerate().skip(start_row + 1) {
            for (col, &cell_value) in row_values.iter().enumerate() {
                if cell_value == '.' {
                    return Some((row, col));
                }
            }
        }

        // Not found on any of the lines
        None
    }

    fn get_candidates(board: &[Vec<char>], row: usize, col: usize) -> BTreeSet<char> {
        let mut result = BTreeSet::new();

        // Add all as possibilities
        for c in '1'..='9' {
            result.insert(c);
        }

        // Remove row and column
        for i in 0..9 {
            result.remove(&board[row][i]);
            result.remove(&board[i][col]);
        }

        // Remove 3x3
        let start_row = (row / 3) * 3;
        let start_col = (col / 3) * 3;
        for row_values in board.iter().skip(start_row).take(3) {
            for cell_value in row_values.iter().skip(start_col).take(3) {
                result.remove(cell_value);
            }
        }

        result
    }

    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        if let Some((unfilled_row, unfilled_col)) = Self::get_next_unfilled_cell(board, 0, 0) {
            Self::solve_sudoku_helper(board, unfilled_row, unfilled_col);
        }
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        [
            ["5","3",".",".","7",".",".",".","."],
            ["6",".",".","1","9","5",".",".","."],
            [".","9","8",".",".",".",".","6","."],
            ["8",".",".",".","6",".",".",".","3"],
            ["4",".",".","8",".","3",".",".","1"],
            ["7",".",".",".","2",".",".",".","6"],
            [".","6",".",".",".",".","2","8","."],
            [".",".",".","4","1","9",".",".","5"],
            [".",".",".",".","8",".",".","7","9"]
        ],
        [
            ["5","3","4","6","7","8","9","1","2"],
            ["6","7","2","1","9","5","3","4","8"],
            ["1","9","8","3","4","2","5","6","7"],
            ["8","5","9","7","6","1","4","2","3"],
            ["4","2","6","8","5","3","7","9","1"],
            ["7","1","3","9","2","4","8","5","6"],
            ["9","6","1","5","3","7","2","8","4"],
            ["2","8","7","4","1","9","6","3","5"],
            ["3","4","5","2","8","6","1","7","9"]
        ]
    )]
    #[case(
        [
            ["5","3","4","6","7","8","9","1","2"],
            ["6","7","2","1","9","5","3","4","8"],
            ["1","9","8","3","4","2","5","6","7"],
            ["8","5","9","7","6","1","4","2","3"],
            ["4","2","6","8","5","3","7","9","1"],
            ["7","1","3","9","2","4","8","5","6"],
            ["9","6","1","5","3","7","2","8","4"],
            ["2","8","7","4","1","9","6","3","5"],
            ["3","4","5","2","8","6","1","7","9"]
        ],
        [
            ["5","3","4","6","7","8","9","1","2"],
            ["6","7","2","1","9","5","3","4","8"],
            ["1","9","8","3","4","2","5","6","7"],
            ["8","5","9","7","6","1","4","2","3"],
            ["4","2","6","8","5","3","7","9","1"],
            ["7","1","3","9","2","4","8","5","6"],
            ["9","6","1","5","3","7","2","8","4"],
            ["2","8","7","4","1","9","6","3","5"],
            ["3","4","5","2","8","6","1","7","9"]
        ]
    )]
    fn case(#[case] input: [[&str; 9]; 9], #[case] expected: [[&str; 9]; 9]) {
        let mut input = input
            .map(|x| x.map(|y| y.chars().next().unwrap()).into())
            .into();
        let expected: Vec<Vec<char>> = expected
            .map(|x| x.map(|y| y.chars().next().unwrap()).into())
            .into();
        Solution::solve_sudoku(&mut input);
        assert_eq!(input, expected);
    }
}
