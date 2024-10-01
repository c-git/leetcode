//! Solution for https://leetcode.com/problems/valid-anagram
//! 242. Valid Anagram

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s: Vec<char> = s.chars().collect();
        s.sort_unstable();
        let mut t: Vec<char> = t.chars().collect();
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
