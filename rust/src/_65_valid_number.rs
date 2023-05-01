enum DecimalOrIntState {
    Start,
    WholeNumber,
    FractionOpt,
    FractionReq,
}

enum IntState {
    Start,
    WholeNumber,
}

impl Solution {
    pub fn is_number(s: String) -> bool {
        // Must be either a decimal or an integer for the mantissa
        let remainder = match Self::is_decimal_or_int(&s[..]) {
            Ok(value) => value,
            Err(e) => {
                if cfg!(debug_assertions) {
                    eprintln!("Invalid Number: {e}")
                }
                return false;
            }
        };

        // Must be an 'e' or 'E'
        match remainder.chars().next() {
            Some(c) => {
                if c != 'e' && c != 'E' {
                    //unexpected character
                    return false;
                }
            }
            None => return true, // early exit no more string
        }

        // Must be an integer with no remaining string as exponent
        Self::is_integer(&remainder[1..])
    }

    fn is_integer(s: &str) -> bool {
        if s.is_empty() {
            return false;
        }
        let mut state = IntState::Start;
        for c in s.chars() {
            match state {
                IntState::Start => match c {
                    '+' | '-' => {
                        if s.len() > 1 {
                            state = IntState::WholeNumber
                        } else {
                            return false;
                        }
                    }
                    '0'..='9' => state = IntState::WholeNumber,
                    _ => return false,
                },
                IntState::WholeNumber => match c {
                    '0'..='9' => (),
                    _ => return false,
                },
            }
        }
        true
    }

    fn is_decimal_or_int(s: &str) -> Result<&str, String> {
        use DecimalOrIntState as State;
        if s.is_empty() {
            return Err("Input is empty".to_string());
        }
        let mut state = State::Start;

        // Remove sign if it exists
        let first_chart = s.chars().next().unwrap();
        let start_index = if first_chart == '+' || first_chart == '-' {
            1
        } else {
            0
        };

        for (i, c) in s.chars().enumerate().skip(start_index) {
            match state {
                State::Start => {
                    state = match c {
                        '0'..='9' => State::WholeNumber,
                        '.' => State::FractionReq,
                        _ => return Err(format!("Expected a number of dot but got {c}")),
                    }
                }
                State::WholeNumber => match c {
                    '0'..='9' => (),
                    '.' => state = State::FractionOpt,
                    _ => return Ok(&s[i..]),
                },
                State::FractionOpt => match c {
                    '0'..='9' => (),
                    _ => return Ok(&s[i..]),
                },
                State::FractionReq => {
                    state = match c {
                        '0'..='9' => State::FractionOpt,
                        _ => return Err(format!("Got a dot so expected a number but got {c}")),
                    }
                }
            }
        }

        match state {
            State::Start => Err("No number found".to_string()),
            State::WholeNumber | State::FractionOpt => Ok(""),
            State::FractionReq => {
                Err("Expected a number of the . but reached the end of the string".to_string())
            }
        }
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = "0".to_string();
        let expected = true;
        let actual = Solution::is_number(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case2() {
        let input = "e".to_string();
        let expected = false;
        let actual = Solution::is_number(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case3() {
        let input = "0".to_string();
        let expected = true;
        let actual = Solution::is_number(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case4() {
        let input = ".".to_string();
        let expected = false;
        let actual = Solution::is_number(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case5() {
        let input = "+".to_string();
        let expected = false;
        let actual = Solution::is_number(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case_valid() {
        let inputs = [
            "2",
            "0089",
            "-0.1",
            "+3.14",
            "4.",
            "-.9",
            "2e10",
            "-90E3",
            "3e+7",
            "+6e-1",
            "53.5e93",
            "-123.456e789",
        ];
        for s in inputs {
            let input = s.to_string();
            let expected = true;
            let actual = Solution::is_number(input);
            assert_eq!(actual, expected, "Failed input is: {s}");
        }
    }

    #[test]
    fn case_invalid() {
        let inputs = ["abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53"];
        for s in inputs {
            let input = s.to_string();
            let expected = false;
            let actual = Solution::is_number(input);
            assert_eq!(actual, expected, "Failed input is: {s}");
        }
    }
}
