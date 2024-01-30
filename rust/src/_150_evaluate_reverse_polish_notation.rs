//! Solution for https://leetcode.com/problems/evaluate-reverse-polish-notation
//! 150. Evaluate Reverse Polish Notation

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut operand_stack = vec![];
        for token in tokens {
            match &token[..] {
                "+" => {
                    let (operand1, operand2) = get_operands(&mut operand_stack);
                    let sum = operand1 + operand2;
                    operand_stack.push(sum);
                }
                "-" => {
                    let (operand1, operand2) = get_operands(&mut operand_stack);
                    let difference = operand1 - operand2;
                    operand_stack.push(difference);
                }
                "*" => {
                    let (operand1, operand2) = get_operands(&mut operand_stack);
                    let product = operand1 * operand2;
                    operand_stack.push(product);
                }
                "/" => {
                    let (operand1, operand2) = get_operands(&mut operand_stack);
                    let dividend = operand1 / operand2;
                    operand_stack.push(dividend);
                }
                _ => operand_stack.push(token.parse().unwrap()), // Assumed to be a number if it's not an operator
            }
        }
        debug_assert_eq!(operand_stack.len(), 1);
        operand_stack
            .pop()
            .expect("Should have exactly 1 item on the stack if input is valid")
    }
}

fn get_operands(operand_stack: &mut Vec<i32>) -> (i32, i32) {
    let operand2 = operand_stack
        .pop()
        .expect("Should have items on stack if RPN is valid");
    let operand1 = operand_stack
        .pop()
        .expect("Should have items on stack if RPN is valid");
    (operand1, operand2)
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
