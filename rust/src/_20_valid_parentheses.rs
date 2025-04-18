//! Solution for https://leetcode.com/problems/valid-parentheses
//! 20. Valid Parentheses

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut open_stack = vec![];
        for c in s.chars() {
            match c {
                ')' => {
                    if Some('(') != open_stack.pop() {
                        return false;
                    }
                }
                ']' => {
                    if Some('[') != open_stack.pop() {
                        return false;
                    }
                }
                '}' => {
                    if Some('{') != open_stack.pop() {
                        return false;
                    }
                }
                _ => open_stack.push(c),
            }
        }
        open_stack.is_empty()
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
