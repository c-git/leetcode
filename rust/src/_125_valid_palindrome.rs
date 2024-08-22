//! Solution for https://leetcode.com/problems/valid-palindrome
//! 125. Valid Palindrome

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s: Vec<char> = s.to_lowercase().chars().collect();
        let mut left = 0;
        let mut right = s.len() - 1;
        while left < right {
            if !s[left].is_alphanumeric() {
                left += 1;
                continue;
            }
            if !s[right].is_alphanumeric() {
                right -= 1;
                continue;
            }
            if s[left] != s[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
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
