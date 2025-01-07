//! Solution for https://leetcode.com/problems/valid-parentheses
//! 20. Valid Parentheses

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for c in s.chars() {
            match c {
                '(' | '[' | '{' => stack.push(c),
                ')' => {
                    if !expect_top_char('(', &mut stack) {
                        return false;
                    }
                }
                ']' => {
                    if !expect_top_char('[', &mut stack) {
                        return false;
                    }
                }
                '}' => {
                    if !expect_top_char('{', &mut stack) {
                        return false;
                    }
                }
                _ => unreachable!("problem guarantee: s consists of parentheses only '()[]{{}}'"),
            }
        }

        stack.is_empty()
    }
}

fn expect_top_char(expected_char: char, stack: &mut Vec<char>) -> bool {
    Some(expected_char) == stack.pop()
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("()", true)]
    #[case("()[]{}", true)]
    #[case("(]", false)]
    #[case("([])", true)]
    fn case(#[case] s: String, #[case] expected: bool) {
        let actual = Solution::is_valid(s);
        assert_eq!(actual, expected);
    }
}
