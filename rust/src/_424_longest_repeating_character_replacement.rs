//! Solution for https://leetcode.com/problems/longest-repeating-character-replacement
//! 424. Longest Repeating Character Replacement

impl Solution {
    /// Based on https://www.youtube.com/watch?v=gqXU1UyA8pk
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let k = k as usize;
        let mut result = 0;
        let mut freqs = [0; 26];
        let mut max_freq = 0;
        let mut left_char_idx = 0;
        for (right_char_idx, right_val) in s.iter().enumerate() {
            let right_freq_idx = char_to_idx(right_val);
            freqs[right_freq_idx] += 1;
            max_freq = max_freq.max(freqs[right_freq_idx]);
            while right_char_idx - left_char_idx + 1 - max_freq > k {
                freqs[char_to_idx(&s[left_char_idx])] -= 1;
                left_char_idx += 1;
            }
            result = result.max(right_char_idx - left_char_idx + 1);
        }
        result as i32
    }
}

fn char_to_idx(char: &u8) -> usize {
    (char - b'A') as usize
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
