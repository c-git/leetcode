//! Solution for https://leetcode.com/problems/valid-parentheses
//! 20. Valid Parentheses

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for c in s.chars() {
            if ['[', '(', '{'].contains(&c) {
                stack.push(c);
                continue;
            }

            let expected = match c {
                ']' => '[',
                ')' => '(',
                '}' => '{',
                _ => unreachable!("by constraint in question regarding valid input"),
            };

            if let Some(top) = stack.pop() {
                if expected != top {
                    return false;
                }
            } else {
                return false;
            }
        }
        stack.is_empty()
    }
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
