enum State {
    Start,
    ReadingNumbers,
}
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut result: u32 = 0;
        let mut is_positive = true;
        let mut state = State::Start;

        for c in s.chars() {
            match state {
                State::Start => {
                    // Read in and ignore any leading whitespace.
                    // Check if the next character (if not already at the end of the string) is '-' or '+'.
                    // Read this character in if it is either.
                    // This determines if the final result is negative or positive respectively.
                    // Assume the result is positive if neither is present.
                    match c {
                        '-' => {
                            is_positive = false;
                            state = State::ReadingNumbers;
                        }
                        '+' => {
                            state = State::ReadingNumbers;
                        }
                        ' ' => (),
                        '0'..='9' => {
                            result = c.to_digit(10).unwrap();
                            state = State::ReadingNumbers;
                        }
                        _ => {
                            break;
                        }
                    }
                }
                State::ReadingNumbers => {
                    //Read in next the characters until the next non-digit character or the end of the input is reached. The rest of the string is ignored.
                    //Convert these digits into an integer (i.e. "123" -> 123, "0032" -> 32). If no digits were read, then the integer is 0. Change the sign as necessary (from step 2).
                    //If the integer is out of the 32-bit signed integer range, then clamp the integer so that it remains in the range.
                    match c {
                        '0'..='9' => result = result * 10 + c.to_digit(10).unwrap(),
                        _ => break,
                    }
                }
            }
        }

        let result = result as i64;

        if is_positive {
            if result > i32::MAX as i64 {
                i32::MAX
            } else {
                result as i32
            }
        } else {
            let temp = 0 - result;
            if temp < i32::MIN as i64 {
                i32::MIN
            } else {
                temp as i32
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
        let input = "42".to_owned();
        let expected = 42;
        let actual = Solution::my_atoi(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case2() {
        let input = "   -42".to_owned();
        let expected = -42;
        let actual = Solution::my_atoi(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case3() {
        let input = "4193 with words".to_owned();
        let expected = 4193;
        let actual = Solution::my_atoi(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case4() {
        let input = "words and 987".to_owned();
        let expected = 0;
        let actual = Solution::my_atoi(input);
        assert_eq!(actual, expected);
    }
}
