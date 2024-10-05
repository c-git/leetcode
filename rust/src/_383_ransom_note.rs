//! Solution for https://leetcode.com/problems/ransom-note
//! 383. Ransom Note

use std::collections::BTreeMap;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        // Also had an idea to use less memory by sorting the bytes and using an in order removal but was less general and not warranted

        // Build up a list of the characters available
        let mut available_chars: BTreeMap<char, usize> = BTreeMap::new();
        for char in magazine.chars() {
            *available_chars.entry(char).or_default() += 1;
        }

        // Check for required characters
        for char in ransom_note.chars() {
            if let Some(qty_left) = available_chars.get_mut(&char) {
                if qty_left > &mut 0 {
                    *qty_left -= 1;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }

        // All required characters found
        true
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("a", "b", false)]
    #[case("aa", "ab", false)]
    #[case("aa", "aab", true)]
    fn case(#[case] ransom_note: String, #[case] magazine: String, #[case] expected: bool) {
        let actual = Solution::can_construct(ransom_note, magazine);
        assert_eq!(actual, expected);
    }
}
