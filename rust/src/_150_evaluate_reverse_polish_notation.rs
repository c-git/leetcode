//! Solution for https://leetcode.com/problems/evaluate-reverse-polish-notation
//! 150. Evaluate Reverse Polish Notation

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        Self::eval_rpn_(tokens).0
    }

    /// Returns the value and any unused tokens
    pub fn eval_rpn_(mut tokens: Vec<String>) -> (i32, Vec<String>) {
        let token = tokens.pop().expect("because of constraint: The input represents a valid arithmetic expression in a reverse polish notation.");

        let is_operator = ["+", "-", "*", "/"].contains(&&token[..]);
        if is_operator {
            let (operand2, rest) = Self::eval_rpn_(tokens);
            let (operand1, rest) = Self::eval_rpn_(rest);
            let value = match &token[..] {
                "+" => operand1 + operand2,
                "-" => operand1 - operand2,
                "/" => operand1 / operand2,
                "*" => operand1 * operand2,
                _ => unreachable!("already checked that it was an operator"),
            };
            (value, rest)
        } else {
            (token.parse().expect("not operator must be operand"), tokens)
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
    #[case(vec!["2".into(),"1".into(),"+".into(),"3".into(),"*".into()], 9)]
    #[case(vec!["4".into(),"13".into(),"5".into(),"/".into(),"+".into()], 6)]
    #[case(vec!["10".into(),"6".into(),"9".into(),"3".into(),"+".into(),"-11".into(),"*".into(),"/".into(),"*".into(),"17".into(),"+".into(),"5".into(),"+".into()], 22)]
    fn case(#[case] tokens: Vec<String>, #[case] expected: i32) {
        let actual = Solution::eval_rpn(tokens);
        assert_eq!(actual, expected);
    }
}
