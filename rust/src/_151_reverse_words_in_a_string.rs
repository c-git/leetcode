//! Solution for https://leetcode.com/problems/reverse-words-in-a-string
//! 151. Reverse Words in a String

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut result = String::new();
        let mut word_end = None;
        let s = s.as_bytes();
        for (i, c) in s.iter().copied().enumerate().rev() {
            if c == b' ' {
                if let Some(end) = word_end {
                    if !result.is_empty() {
                        result.push(' ');
                    }
                    result.push_str(std::str::from_utf8(&s[i + 1..=end]).unwrap());
                    word_end = None;
                }
            } else if word_end.is_none() {
                word_end = Some(i);
            }
        }

        if let Some(end) = word_end {
            if !result.is_empty() {
                result.push(' ');
            }
            result.push_str(std::str::from_utf8(&s[0..=end]).unwrap());
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
    #[case("the sky is blue", "blue is sky the")]
    #[case("  hello world  ", "world hello")]
    #[case("a good   example", "example good a")]
    #[case("a", "a")]
    #[case("a   ", "a")]
    #[case("   a   ", "a")]
    #[case("   a", "a")]
    fn case(#[case] s: String, #[case] expected: String) {
        let actual = Solution::reverse_words(s);
        assert_eq!(actual, expected);
    }
}
