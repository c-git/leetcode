//! Solution for https://leetcode.com/problems/wildcard-matching
//! 44. Wildcard Matching

use std::collections::HashMap;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut memo = HashMap::new();
        Self::is_match_(&s, &p, &mut memo)
    }
    fn is_match_(s: &str, p: &str, memo: &mut HashMap<(usize, usize), bool>) -> bool {
        if s.is_empty() {
            if p.is_empty() {
                return true;
            } else {
                for c in p.chars() {
                    if c != '*' {
                        return false;
                    }
                }
                return true;
            }
        } else if p.is_empty() {
            // s is not empty but p is empty
            return false;
        }

        let key = (s.len(), p.len());
        if let Some(result) = memo.get(&key) {
            return *result;
        }

        let s0 = s.chars().next().unwrap();
        let p0 = p.chars().next().unwrap();

        let result = match (s0, p0) {
            (x, y) if x == y => Self::is_match_(&s[1..], &p[1..], memo),
            (_, '?') => Self::is_match_(&s[1..], &p[1..], memo),
            (_, '*') => {
                Self::is_match_(&s[1..], &p[1..], memo)
                    || Self::is_match_(&s[1..], p, memo)
                    || Self::is_match_(s, &p[1..], memo)
            }
            _ => false,
        };
        memo.insert(key, result);
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
    fn case(#[case] s: String, #[case] p: String, #[case] expected: bool) {
        let actual = Solution::is_match(s, p);
        assert_eq!(actual, expected);
    }
}
