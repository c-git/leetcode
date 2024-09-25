//! Solution for https://leetcode.com/problems/valid-palindrome
//! 125. Valid Palindrome

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut left = s.char_indices();
        let mut right = s.char_indices().rev();
        let mut left_next = left.next();
        let mut right_next = right.next();
        while let (Some(&(l_idx, l_char)), Some(&(r_idx, r_char))) =
            (left_next.as_ref(), right_next.as_ref())
        {
            if l_idx >= r_idx {
                break;
            }
            if !l_char.is_alphanumeric() {
                left_next = left.next();
                continue;
            }
            if !r_char.is_alphanumeric() {
                right_next = right.next();
                continue;
            }
            if l_char.to_ascii_lowercase() != r_char.to_ascii_lowercase() {
                return false;
            }
            left_next = left.next();
            right_next = right.next();
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
