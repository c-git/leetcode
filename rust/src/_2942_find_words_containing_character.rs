//! Solution for https://leetcode.com/problems/find-words-containing-character
//! 2942. Find Words Containing Character

impl Solution {
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        words
            .iter()
            .enumerate()
            .filter_map(|(index, word)| {
                if word.contains(x) {
                    Some(index as i32)
                } else {
                    None
                }
            })
            .collect()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec!["leet".into(),"code".into()], 'e', vec![0,1])]
    #[case(vec!["abc".into(),"bcd".into(),"aaaa".into(),"cbc".into()], 'a', vec![0,2])]
    #[case(vec!["abc".into(),"bcd".into(),"aaaa".into(),"cbc".into()], 'z', vec![])]
    fn case(#[case] words: Vec<String>, #[case] x: char, #[case] expected: Vec<i32>) {
        let actual = Solution::find_words_containing(words, x);
        assert_eq!(actual, expected);
    }
}
