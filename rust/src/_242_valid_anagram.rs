//! Solution for https://leetcode.com/problems/valid-anagram
//! 242. Valid Anagram

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s: Vec<u8> = s.as_bytes().into();
        let mut t: Vec<u8> = t.as_bytes().into();
        s.sort_unstable();
        t.sort_unstable();
        s == t
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("anagram", "nagaram", true)]
    #[case("rat", "car", false)]
    fn case(#[case] s: String, #[case] t: String, #[case] expected: bool) {
        let actual = Solution::is_anagram(s, t);
        assert_eq!(actual, expected);
    }
}
