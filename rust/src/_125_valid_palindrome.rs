//! Solution for https://leetcode.com/problems/valid-palindrome
//! 125. Valid Palindrome

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s: String = s
            .to_lowercase()
            .chars()
            .filter(|x| x.is_alphanumeric())
            .collect();
        let s_rev = s.chars().rev().collect::<String>();
        s == s_rev
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("A man, a plan, a canal: Panama", true)]
    #[case("race a car", false)]
    #[case(" ", true)]
    fn case(#[case] s: String, #[case] expected: bool) {
        let actual = Solution::is_palindrome(s);
        assert_eq!(actual, expected);
    }
}
