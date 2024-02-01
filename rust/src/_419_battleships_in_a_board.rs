//! Solution for https://leetcode.com/problems/battleships-in-a-board
//! 419. Battleships in a Board

impl Solution {
    pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
        let mut result = 0;
        for row in 0..board.len() {
            for col in 0..board[0].len() {
                let c = board[row][col];
                match c {
                    '.' => {}
                    'X' => {
                        // Check if this is a new boat
                        if
                        // Check above
                        (row == 0 || board[row-1][col] == '.') &&
                        // Check on left
                        (col == 0 || board[row][col-1]=='.')
                        {
                            result += 1;
                        }
                    }
                    _ => unreachable!("By problem constraints"),
                }
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
    #[case(vec![vec!['X','.','.','X'],vec!['.','.','.','X'],vec!['.','.','.','X']], 2)]
    #[case(vec![vec!['.']], 0)]
    #[case(vec![vec!['X']], 1)]
    fn case(#[case] board: Vec<Vec<char>>, #[case] expected: i32) {
        let actual = Solution::count_battleships(board);
        assert_eq!(actual, expected);
    }
}
