//! Solution for https://leetcode.com/problems/sum-of-prefix-scores-of-strings
//! 2416. Sum of Prefix Scores of Strings

use std::{collections::BTreeMap, str::Chars};

#[derive(Default, Debug)]
struct Trie {
    root_chars: BTreeMap<char, Box<Trie>>,
    /// Number of words it includes
    count: usize,
    // Don't need to store if this is the end of a word because we don't use it
}

impl Trie {
    fn insert(&mut self, mut word: Chars<'_>) {
        self.count += 1;
        let Some(next_char) = word.next() else {
            // No more characters
            return;
        };
        self.root_chars.entry(next_char).or_default().insert(word);
    }
    fn calc_score(&self, mut word: Chars<'_>) -> usize {
        let Some(next_char) = word.next() else {
            // No more characters
            return 0;
        };

        let next_trie = self
            .root_chars
            .get(&next_char)
            .expect("the word we are looking for was inserted so there must be a next trie because there are more characters");

        // All in the node pull any more that match
        next_trie.count + next_trie.calc_score(word)
    }
}

impl Solution {
    pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
        let mut result = Vec::with_capacity(words.len());
        let trie = Self::build_trie(&words);

        // Test prefixes in trie
        for word in words {
            result.push(trie.calc_score(word.chars()) as _);
        }

        result
    }

    fn build_trie(words: &[String]) -> Trie {
        let mut result = Trie::default();
        for word in words {
            result.insert(word.chars());
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
    #[case(vec!["abc".into(),"ab".into(),"bc".into(),"b".into()], vec![5,4,3,2])]
    #[case(vec!["abc".into(),"ab".into(),"bc".into(),"b".into(), "abc".into()], vec![8,6,3,2,8])]
    #[case(vec!["abcd".into()], vec![4])]
    fn case(#[case] words: Vec<String>, #[case] expected: Vec<i32>) {
        let actual = Solution::sum_prefix_scores(words);
        assert_eq!(actual, expected);
    }
}
