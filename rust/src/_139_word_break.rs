//! Solution for https://leetcode.com/problems/word-break
//! 139. Word Break

use std::collections::HashMap;

struct Trie {
    root: TrieNode,
}
impl Trie {
    fn new(word_dict: Vec<String>) -> Self {
        let mut result = Self {
            root: TrieNode::default(),
        };
        for word in word_dict {
            result.add_word(word);
        }
        result
    }

    fn add_word(&mut self, word: String) {
        let mut node = &mut self.root;
        for c in word.chars() {
            node = node.next.entry(c).or_default();
        }
        node.is_end_of_word = true;
    }
}

#[derive(Default)]
struct TrieNode {
    next: HashMap<char, TrieNode>,
    is_end_of_word: bool,
}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let chars: Vec<char> = s.chars().collect();
        let mut resume_points = vec![0];
        let trie = Trie::new(word_dict);

        while let Some(start_index) = resume_points.pop() {
            let mut node = &trie.root;
            let mut is_fail = false;
            for (i, c) in chars.iter().enumerate().skip(start_index) {
                if node.is_end_of_word {
                    resume_points.push(i);
                }
                if let Some(next_node) = node.next.get(c) {
                    node = next_node;
                } else {
                    // Not able to continue check for next resume point
                    is_fail = true;
                    break;
                };
            }
            if !is_fail && node.is_end_of_word {
                // Reached to end without failing all characters are part of a word
                return true;
            }
        }

        // No more resume points, unable to complete chars
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
    #[case("applespenapple", vec!["apple".into(),"pen".into(),"apples".into()], true)]
    #[case("aaaaaaa", vec!["aaaa".into(),"aa".into()], false)]
    fn case(#[case] s: String, #[case] word_dict: Vec<String>, #[case] expected: bool) {
        let actual = Solution::word_break(s, word_dict);
        assert_eq!(actual, expected);
    }
}
