//! Solution for https://leetcode.com/problems/number-of-substrings-containing-all-three-characters
//! 1358. Number of Substrings Containing All Three Characters

impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        let mut result = 0;
        let mut counts = [0; 3];
        let mut left = 0;
        for (right, c) in s.iter().copied().enumerate() {
            Self::update_counts(&mut counts, c, true);
            while counts.iter().all(|&x| x > 0) {
                debug_assert!(left < right);
                result += s.len() - right;
                Self::update_counts(&mut counts, s[left], false);
                left += 1;
            }
        }
        result as _
    }

    fn update_counts(counts: &mut [u16; 3], char: u8, should_inc: bool) {
        let index = match char {
            b'a' => 0,
            b'b' => 1,
            b'c' => 2,
            _ => unreachable!("s only consists of a, b or c characters"),
        };
        if should_inc {
            counts[index] += 1;
        } else {
            counts[index] -= 1;
        }
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("abcabc", 10)]
    #[case("aaacb", 3)]
    #[case("abc", 1)]
    fn case(#[case] s: String, #[case] expected: i32) {
        let actual = Solution::number_of_substrings(s);
        assert_eq!(actual, expected);
    }
}
