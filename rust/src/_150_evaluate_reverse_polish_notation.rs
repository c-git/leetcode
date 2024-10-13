//! Solution for https://leetcode.com/problems/evaluate-reverse-polish-notation
//! 150. Evaluate Reverse Polish Notation

enum Value {
    Add,
    Sub,
    Div,
    Mul,
    Num(i32),
}

impl Value {
    /// Returns `true` if the value is an operator.
    #[must_use]
    fn is_operator(&self) -> bool {
        !matches!(self, Self::Num(..))
    }
}

impl Solution {
    pub fn eval_rpn(mut tokens: Vec<String>) -> i32 {
        let mut stack = vec![];

        while let Some(token) = tokens.pop() {
            stack.push(match &token[..] {
                "+" => Value::Add,
                "-" => Value::Sub,
                "/" => Value::Div,
                "*" => Value::Mul,
                num => Value::Num(num.parse().expect("not operator must be operand")),
            });

            // Check if we have two operands and an operator on the stack to collapse into one
            while stack.len() >= 3
                && stack[stack.len() - 3].is_operator()
                && !stack[stack.len() - 2].is_operator()
                && !stack[stack.len() - 1].is_operator()
            {
                let Some(Value::Num(operand1)) = stack.pop() else {
                    unreachable!()
                };
                let Some(Value::Num(operand2)) = stack.pop() else {
                    unreachable!()
                };
                let Some(operator) = stack.pop() else {
                    unreachable!()
                };
                stack.push(match operator {
                    Value::Add => Value::Num(operand1 + operand2),
                    Value::Sub => Value::Num(operand1 - operand2),
                    Value::Div => Value::Num(operand1 / operand2),
                    Value::Mul => Value::Num(operand1 * operand2),
                    Value::Num(_) => unreachable!("checked for operator in if statement"),
                });
            }
        }
        debug_assert_eq!(stack.len(), 1);
        let Some(Value::Num(result)) = stack.pop() else {
            panic!("invalid rpn  received")
        };
        result
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
