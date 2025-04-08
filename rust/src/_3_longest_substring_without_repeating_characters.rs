//! Solution for https://leetcode.com/problems/longest-substring-without-repeating-characters
//! 3. Longest Substring Without Repeating Characters

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut result = 0;
        let s = s.as_bytes();
        let mut left_idx = 0;
        let mut freq_count = [0u8; 26];        
        for (right_idx, right_val) in s.iter().enumerate() {
            let right_char_idx = (right_val - b'a') as usize;
            freq_count[right_char_idx] += 1;
            while freq_count[right_char_idx] > 1 {
                freq_count[(s[left_idx] - b'a') as usize] -= 1;
                left_idx += 1;
            }
            result = result.max(right_idx-left_idx+1);
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
