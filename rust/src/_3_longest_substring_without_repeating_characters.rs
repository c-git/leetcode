//! Solution for https://leetcode.com/problems/longest-substring-without-repeating-characters
//! 3. Longest Substring Without Repeating Characters

use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut result = 0;
        let s = s.as_bytes();
        let mut left_idx = 0;
        let mut freq_count: HashMap<u8, u8> = HashMap::new();
        for (right_idx, &right_val) in s.iter().enumerate() {
            *freq_count.entry(right_val).or_default() += 1;
            while *freq_count.get(&right_val).unwrap() > 1 {
                *freq_count.get_mut(&s[left_idx]).unwrap() -= 1;
                left_idx += 1;
            }
            result = result.max(right_idx - left_idx + 1);
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
    #[case("abcabcbb", 3)]
    #[case("bbbbb", 1)]
    #[case("pwwkew", 3)]
    fn case(#[case] s: String, #[case] expected: i32) {
        let actual = Solution::length_of_longest_substring(s);
        assert_eq!(actual, expected);
    }
}
