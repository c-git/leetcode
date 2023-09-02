//! Solution for https://leetcode.com/problems/extra-characters-in-a-string
//! 2707. Extra Characters in a String

use std::collections::HashMap;

struct Trie {
    children: HashMap<char, Trie>,
    is_end_of_word: bool,
}

impl Trie {
    fn new() -> Self {
        Self {
            children: Default::default(),
            is_end_of_word: Default::default(),
        }
    }
    fn add_word(&mut self, word: &str) {
        let mut curr = self;
        for c in word.chars() {
            curr = curr.children.entry(c).or_insert(Trie::new());
        }
        curr.is_end_of_word = true;
    }
}

impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let mut result = i32::MAX;

        let mut full_trie = Trie::new();
        dictionary.iter().for_each(|x| full_trie.add_word(x));

        let s: Vec<char> = s.chars().collect();
        let n = s.len();
        let mut stack = vec![(0, 0, 0, &full_trie)];
        while let Some((idx, unused, letters_matched, trie)) = stack.pop() {
            // LI:
            //    - idx is a valid index into s
            //    - unused holds how many letters have been skipped so far
            //    - letters_matched is how many letters have been matched so far in this current word
            //    - trie is the part of the try remaining after matching previous letters in s[..i]

            let is_last_letter = idx + 1 >= n;
            let is_at_top_of_trie = letters_matched == 0;

            if !is_last_letter && is_at_top_of_trie {
                // Always consider the option of skipping when starting a new match
                stack.push((idx + 1, unused + 1, 0, &full_trie));
            }

            match trie.children.get(&s[idx]) {
                Some(child) => match (child.is_end_of_word, is_last_letter) {
                    (true, true) => {
                        // End of the input letters this is one way to use all the letters
                        if unused == 0 {
                            return 0;
                        } else {
                            result = result.min(unused);
                        }
                    }
                    (true, false) => {
                        // This is a word but there are more letters keep
                        // Both keep on trying to match
                        // and consider starting from the beginning again
                        stack.push((idx + 1, unused, 0, &full_trie));
                        stack.push((idx + 1, unused, letters_matched + 1, child));
                    }
                    (false, true) => (), // letters finished but not the end of a word, unusable path
                    (false, false) => {
                        // Not the end of a word, only option is to move forward
                        stack.push((idx + 1, unused, letters_matched + 1, child));
                    }
                },
                None => {
                    // Unable to match the next letter
                    if is_last_letter {
                        // One more letter that can't be used
                        result = result.min(unused + 1);
                    } else {
                        // Failed path do nothing
                    }
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
    #[case("leetscode", vec!["leet".into(),"code".into(),"leetcode".into()], 1)]
    #[case("sayhelloworld", vec!["hello".into(),"world".into()], 3)]
    #[case("leetsleet", vec!["leet".into()], 1)]
    #[case("leetleeleet", vec!["leet".into(), "lee".into()], 0)]
    #[case("leetleet", vec!["leet".into()], 0)]
    #[case("le", vec!["lee".into()], 2)]
    #[case("leetleeleete", vec!["lee".into(),"leet".into(),"leeleete".into()], 0)]
    #[case("e", vec!["e".into()], 0)]
    #[case("e", vec!["a".into()], 1)]
    #[case("eeeeee", vec!["e".into()], 0)]
    #[case("eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee", vec!["e".into()], 0)]
    fn case(#[case] s: String, #[case] dictionary: Vec<String>, #[case] expected: i32) {
        let actual = Solution::min_extra_char(s, dictionary);
        assert_eq!(actual, expected);
    }
}
