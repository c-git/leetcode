//! Solution for https://leetcode.com/problems/word-break
//! 139. Word Break

use std::collections::HashMap;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let s = s.as_bytes();
        let mut trie = Trie {
            root: Default::default(),
        };
        for word in word_dict.iter() {
            trie.add_word(word);
        }

        let mut continuation_points = vec![0];
        while let Some(start) = continuation_points.pop() {
            let mut curr = &trie.root;
            for i in start..s.len() {
                if let Some(next) = curr.next.get(&s[i]) {
                    if next.is_terminal {
                        if i + 1 == s.len() {
                            return true;
                        }
                        continuation_points.push(i + 1);
                    }
                    curr = next;
                } else {
                    // Unable to match a word anymore
                    break;
                }
            }
        }

        // Out of continuation points unable to match
        false
    }
}

struct Trie {
    root: TrieNode,
}

#[derive(Default)]
struct TrieNode {
    is_terminal: bool,
    next: HashMap<u8, TrieNode>,
}

impl Trie {
    fn add_word(&mut self, word: &str) {
        let mut curr = &mut self.root;
        for c in word.as_bytes() {
            curr = curr.next.entry(*c).or_default();
        }
        curr.is_terminal = true;
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
