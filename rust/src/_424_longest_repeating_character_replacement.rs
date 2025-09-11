//! Solution for https://leetcode.com/problems/longest-repeating-character-replacement
//! 424. Longest Repeating Character Replacement

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let mut result = 0;
        for curr_char in b'A'..=b'Z' {
            let mut left = 0;
            let mut replacements_left = k;
            for right in 0..s.len() {
                if s[right] != curr_char {
                    replacements_left -= 1;
                }
                while replacements_left < 0 {
                    if s[left] != curr_char {
                        replacements_left += 1;
                    }
                    left += 1;
                }
                result = result.max(right - left + 1);
            }
        }
        result as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("ABAB", 2, 4)]
    #[case("AABABBA", 1, 4)]
    fn case(#[case] s: String, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::character_replacement(s, k);
        assert_eq!(actual, expected);
    }
}
