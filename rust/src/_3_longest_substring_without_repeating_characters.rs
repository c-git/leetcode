//! Solution for https://leetcode.com/problems/longest-substring-without-repeating-characters
//! 3. Longest Substring Without Repeating Characters

use std::collections::BTreeSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        // ASSUMPTION: s consists of English letters, digits, symbols and spaces.
        // Assumption allows me to work with bytes instead of characters
        let mut result = 0;
        let mut included = BTreeSet::new();
        let mut start = 0;
        let s = s.as_bytes();
        for (i, c) in s.iter().enumerate() {
            if included.contains(c) {
                // Already included remove up to the last one

                // Move start onto the previous one
                while s[start] != *c {
                    debug_assert!(start<i, "If this is violated something has gone very wrong. The character is 'included' but does not show up");
                    included.remove(&s[start]);
                    start += 1;
                }

                // Move start forward to not include the previous one
                start += 1;
            } else {
                // Add new letter
                included.insert(*c);
                result = result.max(i - start + 1);
            }
        }
        result as _
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("abcabcbb", 3)]
    #[case("bbbbb", 1)]
    #[case("pwwkew", 3)]
    #[case("pwwmawkdew", 6)]
    fn case(#[case] s: String, #[case] expected: i32) {
        let actual = Solution::length_of_longest_substring(s);
        assert_eq!(actual, expected);
    }
}
