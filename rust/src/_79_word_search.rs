//! Solution for https://leetcode.com/problems/word-search
//! 79. Word Search

use std::collections::BTreeSet;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let word: Vec<_> = word.chars().collect();
        for (row_idx, row) in board.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                if cell != &word[0] {
                    continue;
                }
                let mut seen: BTreeSet<(usize, usize)> = BTreeSet::new();
                let pos = (row_idx, col_idx);
                seen.insert(pos);
                if Self::dfs(pos, &board, &word[1..], &mut seen) {
                    return true;
                }
            }
        }
        false
    }

    fn dfs(
        pos: (usize, usize),
        board: &[Vec<char>],
        word: &[char],
        seen: &mut BTreeSet<(usize, usize)>,
    ) -> bool {
        if word.is_empty() {
            return true;
        }

        let mut check_path = |pos_to_check: (usize, usize)| {
            if board[pos_to_check.0][pos_to_check.1] == word[0] && !seen.contains(&pos_to_check) {
                seen.insert(pos_to_check);
                if Self::dfs(pos_to_check, board, &word[1..], seen) {
                    return true;
                }
                seen.remove(&pos_to_check);
            }
            false
        };

        if pos.0 > 0 && check_path((pos.0 - 1, pos.1)) {
            return true;
        }
        if pos.1 > 0 && check_path((pos.0, pos.1 - 1)) {
            return true;
        }
        if pos.0 < board.len() - 1 && check_path((pos.0 + 1, pos.1)) {
            return true;
        }
        if pos.1 < board[0].len() - 1 && check_path((pos.0, pos.1 + 1)) {
            return true;
        }
        false
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']], "ABCCED", true)]
    #[case(vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']], "SEE", true)]
    #[case(vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']], "ABCB", false)]
    fn case(#[case] board: Vec<Vec<char>>, #[case] word: String, #[case] expected: bool) {
        let actual = Solution::exist(board, word);
        assert_eq!(actual, expected);
    }
}
