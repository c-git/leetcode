//! Solution for https://leetcode.com/problems/word-search-ii
//! 212. Word Search II

use std::collections::HashMap;

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut result = Vec::new();
        let mut trie = Trie::default();
        for word in words.iter() {
            trie.add_word(word);
        }

        let mut visited = vec![vec![false; board[0].len()]; board.len()];
        let mut partial_word = String::new();

        for row in 0..board.len() {
            for col in 0..board[0].len() {
                check_for_word(
                    row,
                    col,
                    &board,
                    &mut partial_word,
                    &mut trie.root_node,
                    &mut visited,
                    &mut result,
                );
            }
        }

        result
    }
}

fn check_for_word(
    row: usize,
    col: usize,
    board: &[Vec<char>],
    partial_word: &mut String,
    node: &mut TrieNode,
    visited: &mut [Vec<bool>],
    result: &mut Vec<String>,
) {
    debug_assert!(!visited[row][col]);
    if let Some(next) = node.next_node.get_mut(&(board[row][col] as u8)) {
        visited[row][col] = true;
        partial_word.push(board[row][col]);
        if next.is_terminal {
            result.push(partial_word.to_string());
            next.is_terminal = false;
        }

        // Check Up
        if row > 0 && !visited[row - 1][col] {
            check_for_word(row - 1, col, board, partial_word, next, visited, result);
        }

        // Check Down
        if row < board.len() - 1 && !visited[row + 1][col] {
            check_for_word(row + 1, col, board, partial_word, next, visited, result);
        }

        // Check Left
        if col > 0 && !visited[row][col - 1] {
            check_for_word(row, col - 1, board, partial_word, next, visited, result);
        }

        // Check Right
        if col < board[0].len() - 1 && !visited[row][col + 1] {
            check_for_word(row, col + 1, board, partial_word, next, visited, result);
        }

        partial_word.pop();
        visited[row][col] = false;
    }
}

#[derive(Default)]
struct Trie {
    root_node: TrieNode,
}

#[derive(Default)]
struct TrieNode {
    is_terminal: bool,
    next_node: HashMap<u8, TrieNode>,
}

impl Trie {
    fn add_word(&mut self, word: &str) {
        let mut node = &mut self.root_node;
        for &c in word.as_bytes() {
            node = node.next_node.entry(c).or_default();
        }
        node.is_terminal = true;
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec!['o','a','a','n'],vec!['e','t','a','e'],vec!['i','h','k','r'],vec!['i','f','l','v']], vec!["oath".into(),"pea".into(),"eat".into(),"rain".into()], vec!["eat".into(),"oath".into()])]
    #[case(vec![vec!['a','b'],vec!['c','d']], vec!["abcb".into()], vec![])]
    fn case(
        #[case] board: Vec<Vec<char>>,
        #[case] words: Vec<String>,
        #[case] mut expected: Vec<String>,
    ) {
        let mut actual = Solution::find_words(board, words);
        actual.sort();
        expected.sort();
        assert_eq!(actual, expected);
    }
}
