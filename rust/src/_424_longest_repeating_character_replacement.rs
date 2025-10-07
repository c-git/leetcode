//! Solution for https://leetcode.com/problems/longest-repeating-character-replacement
//! 424. Longest Repeating Character Replacement

impl Solution {
    /// Based on https://www.youtube.com/watch?v=gqXU1UyA8pk
    /// Allowed me to realize I was on the right path and changed the code only remove one character at a time
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let k = k as usize;
        let mut result = 0;
        let mut left = 0;
        let mut freq = [0; 26];
        let mut max = 0;
        for (right, c) in s.iter().enumerate() {
            let right_char_idx = (c - b'A') as usize;
            freq[right_char_idx] += 1;
            max = max.max(freq[right_char_idx]);
            while (right - left + 1) - max > k {
                let left_char_idx = (s[left] - b'A') as usize;
                freq[left_char_idx] -= 1;
                left += 1;
                max = *freq.iter().max().expect("freq is always 26 long");
            }
            result = result.max(right - left + 1);
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
