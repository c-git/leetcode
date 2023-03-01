struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        let open_brackets = ['(', '{', '['];
        for c in s.chars() {
            if open_brackets.contains(&c) {
                stack.push(c);
            } else {
                match c {
                    ')' => {
                        if !Self::check_pop('(', &mut stack) {
                            return false;
                        }
                    }
                    '}' => {
                        if !Self::check_pop('{', &mut stack) {
                            return false;
                        }
                    }
                    ']' => {
                        if !Self::check_pop('[', &mut stack) {
                            return false;
                        }
                    }

                    _ => unreachable!("Assumed only brackets based on question"),
                }
            }
        }
        stack.is_empty() // If stack is empty then string is valid otherwise there are unclosed brackets
    }

    fn check_pop(open_bracket: char, stack: &mut Vec<char>) -> bool {
        if !stack.is_empty() && stack[stack.len() - 1] == open_bracket {
            stack.pop();
            return true;
        }

        // Wrong char at top of stack or stack empty
        false
    }
}

#[test]
fn case1() {
    let s = "()".to_string();
    let expected = true;

    let actual = Solution::is_valid(s);
    assert_eq!(actual, expected);
}

#[test]
fn case2() {
    let s = "()[]{}".to_string();
    let expected = true;

    let actual = Solution::is_valid(s);
    assert_eq!(actual, expected);
}

#[test]
fn case3() {
    let s = "(]".to_string();
    let expected = false;

    let actual = Solution::is_valid(s);
    assert_eq!(actual, expected);
}
