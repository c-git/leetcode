//! Solution for https://leetcode.com/problems/wildcard-matching
//! 44. Wildcard Matching

use std::vec;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut memo = vec![vec![None; p.len() + 1]; s.len() + 1];
        Self::is_match_(s.as_bytes(), p.as_bytes(), &mut memo)
    }
    fn is_match_(s: &[u8], p: &[u8], memo: &mut Vec<Vec<Option<bool>>>) -> bool {
        if s.is_empty() {
            if p.is_empty() {
                return true;
            } else {
                for &c in p {
                    if c != b'*' {
                        return false;
                    }
                }
                return true;
            }
        } else if p.is_empty() {
            // s is not empty but p is empty
            return false;
        }

        if let Some(result) = memo[s.len()][p.len()] {
            return result;
        }

        let result = match (s[0], p[0]) {
            (x, y) if x == y => Self::is_match_(&s[1..], &p[1..], memo),
            (_, b'?') => Self::is_match_(&s[1..], &p[1..], memo),
            (_, b'*') => {
                Self::is_match_(&s[1..], &p[1..], memo)
                    || Self::is_match_(&s[1..], p, memo)
                    || Self::is_match_(s, &p[1..], memo)
            }
            _ => false,
        };
        memo[s.len()][p.len()] = Some(result);
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
    #[case("aa", "a", false)]
    #[case("aa", "*", true)]
    #[case("cb", "?a", false)]
    #[case("aa", "b*", false)]
    #[case("", "*", true)]
    #[case("aa", "aa*", true)]
    #[case("ab", "aa*", false)]
    #[case("hello", "h?*", true)]
    #[case("adceb", "*a*b*", true)]
    #[case("abcedefghijklmnopqrs", "**************a*g*s*", true)]
    fn case(#[case] s: String, #[case] p: String, #[case] expected: bool) {
        let actual = Solution::is_match(s, p);
        assert_eq!(actual, expected);
    }
}
