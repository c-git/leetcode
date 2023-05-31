struct SeenTracker {
    data: [bool; 9],
}
impl SeenTracker {
    fn new() -> Self {
        Self {
            data: Default::default(),
        }
    }

    fn reset(&mut self) {
        for element in &mut self.data {
            *element = false;
        }
    }

    /// Sets this value as seen. If it has been seen before returns false
    fn set(&mut self, value: char) -> bool {
        if value == '.' {
            return true; // This is always fine
        }
        let value = value.to_digit(10).unwrap() as usize - 1;
        if !self.data[value] {
            self.data[value] = true;
            true // Check can proceed
        } else {
            false // Already seen check has failed
        }
    }
}
impl Solution {
    #[allow(clippy::needless_range_loop)]
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut seen = SeenTracker::new();
        // Check Columns
        for col in 0..9 {
            seen.reset();
            for row in 0..9 {
                if !seen.set(board[row][col]) {
                    return false;
                }
            }
        }

        // Check Rows
        for row in 0..9 {
            seen.reset();
            for col in 0..9 {
                if !seen.set(board[row][col]) {
                    return false;
                }
            }
        }

        // Check 3x3
        for top in (0..9).step_by(3) {
            for left in (0..9).step_by(3) {
                seen.reset();
                for row_offset in 0..3 {
                    for col_offset in 0..3 {
                        let row = top + row_offset;
                        let col = left + col_offset;
                        if !seen.set(board[row][col]) {
                            return false;
                        }
                    }
                }
            }
        }

        // All passed this is fine
        true
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case([["5","3",".",".","7",".",".",".","."],
            ["6",".",".","1","9","5",".",".","."],
            [".","9","8",".",".",".",".","6","."],
            ["8",".",".",".","6",".",".",".","3"],
            ["4",".",".","8",".","3",".",".","1"],
            ["7",".",".",".","2",".",".",".","6"],
            [".","6",".",".",".",".","2","8","."],
            [".",".",".","4","1","9",".",".","5"],
            [".",".",".",".","8",".",".","7","9"]],
            true)]
    #[case([["8","3",".",".","7",".",".",".","."],
            ["6",".",".","1","9","5",".",".","."],
            [".","9","8",".",".",".",".","6","."],
            ["8",".",".",".","6",".",".",".","3"],
            ["4",".",".","8",".","3",".",".","1"],
            ["7",".",".",".","2",".",".",".","6"],
            [".","6",".",".",".",".","2","8","."],
            [".",".",".","4","1","9",".",".","5"],
            [".",".",".",".","8",".",".","7","9"]],
            false)]
    fn case(#[case] input: [[&str; 9]; 9], #[case] expected: bool) {
        let input = input
            .map(|x| x.map(|y| y.chars().next().unwrap()).into())
            .into();
        let actual = Solution::is_valid_sudoku(input);
        assert_eq!(actual, expected);
    }
}
