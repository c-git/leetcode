//! Solution for https://leetcode.com/problems/is-subsequence
//! 392. Is Subsequence

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut iter_s = s.chars();
        let mut char_s = match iter_s.next() {
            Some(c) => c,
            None => return true,
        };
        for char_t in t.chars() {
            if char_t == char_s {
                char_s = match iter_s.next() {
                    Some(c) => c,
                    None => return true,
                };
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
    #[case("abc", "ahbgdc", true)]
    #[case("axc", "ahbgdc", false)]
    fn case(#[case] s: String, #[case] t: String, #[case] expected: bool) {
        let actual = Solution::is_subsequence(s, t);
        assert_eq!(actual, expected);
    }
}
