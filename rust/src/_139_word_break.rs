//! Solution for https://leetcode.com/problems/word-break
//! 139. Word Break

use std::collections::HashMap;

struct Trie {
    root: TrieNode,
}

#[derive(Default)]
struct TrieNode {
    is_terminal: bool,
    next_node: HashMap<u8, TrieNode>,
}

impl Trie {
    fn build(word_dict: &[String]) -> Self {
        let mut result = Self {
            root: TrieNode::default(),
        };

        for word in word_dict {
            let mut curr = &mut result.root;
            for c in word.as_bytes().iter().copied() {
                curr = curr.next_node.entry(c).or_default();
            }
            curr.is_terminal = true;
        }
        result
    }
}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let s = s.as_bytes();
        let mut dp = vec![false; s.len() + 1];
        *dp.last_mut().unwrap() = true;
        let trie = Trie::build(&word_dict);
        for i in (0..s.len()).rev() {
            dp[i] = Self::can_start_at(s, i, &dp, &trie);
        }
        dp[0]
    }

    fn can_start_at(s: &[u8], mut i: usize, dp: &[bool], trie: &Trie) -> bool {
        let mut curr = &trie.root;
        while i < s.len() {
            if let Some(x) = curr.next_node.get(&s[i]) {
                curr = x;
            } else {
                return false;
            }
            i += 1;
            if curr.is_terminal && dp[i] {
                return true;
            }
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
    #[case("leetcode", vec!["leet".into(),"code".into()], true)]
    #[case("applepenapple", vec!["apple".into(),"pen".into()], true)]
    #[case("catsandog", vec!["cats".into(),"dog".into(),"sand".into(),"and".into(),"cat".into()], false)]
    fn case(#[case] s: String, #[case] word_dict: Vec<String>, #[case] expected: bool) {
        let actual = Solution::word_break(s, word_dict);
        assert_eq!(actual, expected);
    }
}
