//! Solution for https://leetcode.com/problems/word-break
//! 139. Word Break

use std::collections::HashSet;

impl Solution {
    /// Based on https://www.youtube.com/watch?v=TK9pptFzH-A
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let word_dict: HashSet<&str> = word_dict.iter().map(|x| x.as_str()).collect();

        // Stores a set containing the possible start points
        let mut dp = HashSet::new();
        dp.insert(0);
        for end_index in 0..s.len() {
            for start_index in dp.iter().copied() {
                if word_dict.contains(&s[start_index..=end_index]) {
                    if end_index == s.len() - 1 {
                        return true; // We made it to the end
                    }
                    dp.insert(end_index + 1);
                    break;
                }
            }
        }

        false // Unable to get to end
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
