//! Solution for https://leetcode.com/problems/word-ladder
//! 127. Word Ladder

use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut seen: HashSet<&str> = HashSet::new();
        let mut graph: HashMap<&str, Vec<&str>> = HashMap::with_capacity(word_list.len());

        // Create mapping of words to other words with one diff
        for (i, word1) in word_list.iter().enumerate() {
            for word2 in word_list.iter().skip(i + 1) {
                if is_one_diff(word1, word2) {
                    graph.entry(word1).or_default().push(word2);
                    graph.entry(word2).or_default().push(word1);
                }
            }
        }

        if !graph.contains_key(end_word.as_str()) {
            // End word not in `word_list`
            return 0;
        }

        let mut queue: VecDeque<(&str, i32)> = VecDeque::new();

        // Find starting nodes
        for word in word_list.iter() {
            if is_one_diff(&begin_word, word) {
                queue.push_back((word.as_str(), 2));
                seen.insert(word);
            }
        }

        // Work through queue to find end word
        while let Some((word, distance)) = queue.pop_front() {
            if word == end_word.as_str() {
                return distance;
            }

            let Some(neighbours) = graph.get(word) else {
                continue;
            };

            // Add neighbours
            for neighbour in neighbours {
                let is_new_insert = seen.insert(neighbour);
                if is_new_insert {
                    queue.push_back((neighbour, distance + 1));
                }
            }
        }

        0 // Not found
    }
}

fn is_one_diff(word1: &str, word2: &str) -> bool {
    let mut found_diff = false;
    for (c1, c2) in word1.chars().zip(word2.chars()) {
        match (c1 != c2, found_diff) {
            (true, true) => return false, // more than one diff
            (true, false) => found_diff = true,
            _ => {} // Nothing to be done
        }
    }
    found_diff
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("hit", "cog", vec!["hot".into(),"dot".into(),"dog".into(),"lot".into(),"log".into(),"cog".into()], 5)]
    #[case("hit", "cog", vec!["hot".into(),"dot".into(),"dog".into(),"lot".into(),"log".into()], 0)]
    fn case(
        #[case] begin_word: String,
        #[case] end_word: String,
        #[case] word_list: Vec<String>,
        #[case] expected: i32,
    ) {
        let actual = Solution::ladder_length(begin_word, end_word, word_list);
        assert_eq!(actual, expected);
    }
}
