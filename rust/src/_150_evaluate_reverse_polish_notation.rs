//! Solution for https://leetcode.com/problems/evaluate-reverse-polish-notation
//! 150. Evaluate Reverse Polish Notation

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        // Based on fastest solution: Realized if I started from the front the stack only has to store numbers
        let mut stack = vec![];

        for token in tokens {
            let is_operator = ["+", "-", "/", "*"].contains(&&token[..]);
            if is_operator {
                let operand2 = stack.pop().unwrap();
                let operand1 = stack.pop().unwrap();

                stack.push(match &token[..] {
                    "+" => operand1 + operand2,
                    "-" => operand1 - operand2,
                    "/" => operand1 / operand2,
                    "*" => operand1 * operand2,
                    _ => unreachable!("already confirmed it was an operator"),
                });
            } else {
                stack.push(token.parse().unwrap());
            }
        }

        debug_assert_eq!(stack.len(), 1);
        stack.pop().unwrap()
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
