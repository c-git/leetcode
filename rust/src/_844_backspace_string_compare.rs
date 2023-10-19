//! Solution for https://leetcode.com/problems/backspace-string-compare
//! 844. Backspace String Compare

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let mut s_clean = vec![];
        let mut t_clean = vec![];
        for c in s.chars() {
            if c == '#' {
                s_clean.pop();
            } else {
                s_clean.push(c);
            }
        }
        for c in t.chars() {
            if c == '#' {
                t_clean.pop();
            } else {
                t_clean.push(c);
            }
        }
        s_clean == t_clean
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("ab#c", "ad#c", true)]
    #[case("ab##", "c#d#", true)]
    #[case("a#c", "b", false)]
    fn case(#[case] s: String, #[case] t: String, #[case] expected: bool) {
        let actual = Solution::backspace_compare(s, t);
        assert_eq!(actual, expected);
    }
}
