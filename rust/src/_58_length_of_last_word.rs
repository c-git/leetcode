//! Solution for https://leetcode.com/problems/length-of-last-word
//! 58. Length of Last Word

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut word_end_from_back = 0i32;
        let mut word_start = s.len() as i32 - 1;
        // Find first non space
        for (i, c) in s.chars().rev().enumerate() {
            if c != ' ' {
                word_end_from_back = i as i32;
                break;
            }
        }

        // Find next space
        for (i, c) in s
            .chars()
            .rev()
            .enumerate()
            .skip(word_end_from_back as usize)
        {
            if c == ' ' {
                word_start = i as i32 - 1;
                break;
            }
        }

        // answer is difference in value
        word_start - word_end_from_back + 1
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("Hello World", 5)]
    #[case("   fly me   to   the moon  ", 4)]
    #[case("luffy is still joyboy", 6)]
    #[case("a", 1)]
    fn case(#[case] s: String, #[case] expected: i32) {
        let actual = Solution::length_of_last_word(s);
        assert_eq!(actual, expected);
    }
}
