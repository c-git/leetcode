//! Solution for https://leetcode.com/problems/construct-smallest-number-from-di-string
//! 2375. Construct Smallest Number From DI String

/// Used to simplify problem into cases
enum State {
    PrevComplete {
        next_pos: usize,
    },
    /// Stores the start of the D's and the next position to check
    FindEndOfD {
        start: usize,
        next_pos: usize,
    },
    /// Stores the last position filled in, the start of the D's, and the position after the last D
    FillInD {
        start: usize,
        last_pos: usize,
        end_of_d: usize,
    },
}

impl Solution {
    pub fn smallest_number(pattern: String) -> String {
        // Over arching idea:
        // - Only focus on where the next number must go (where that is depends on the current state)

        let pattern = pattern.as_bytes();
        let mut result = vec![None; pattern.len() + 1];
        let mut next_char = b'1';

        let mut state = State::PrevComplete { next_pos: 0 };
        loop {
            state = match state {
                State::PrevComplete { next_pos } => {
                    if next_pos >= pattern.len() {
                        result[next_pos] = Some(next_char);
                        break;
                    }
                    if pattern[next_pos] == b'I' {
                        result[next_pos] = Some(next_char);
                        next_char += 1;
                        State::PrevComplete {
                            next_pos: next_pos + 1,
                        }
                    } else {
                        State::FindEndOfD {
                            start: next_pos,
                            next_pos: next_pos + 1,
                        }
                    }
                }
                State::FindEndOfD { start, next_pos } => {
                    if next_pos >= pattern.len() {
                        result[next_pos] = Some(next_char);
                        next_char += 1;
                        State::FillInD {
                            start,
                            last_pos: next_pos,
                            end_of_d: next_pos,
                        }
                    } else if pattern[next_pos] == b'I' {
                        result[next_pos] = Some(next_char);
                        next_char += 1;
                        State::FillInD {
                            start,
                            last_pos: next_pos,
                            end_of_d: next_pos,
                        }
                    } else {
                        State::FindEndOfD {
                            start,
                            next_pos: next_pos + 1,
                        }
                    }
                }
                State::FillInD {
                    start,
                    last_pos,
                    end_of_d,
                } => {
                    if last_pos > start {
                        let curr_pos = last_pos - 1;
                        result[curr_pos] = Some(next_char);
                        next_char += 1;
                        State::FillInD {
                            start,
                            last_pos: curr_pos,
                            end_of_d,
                        }
                    } else {
                        if end_of_d >= pattern.len() {
                            break;
                        }
                        State::PrevComplete {
                            next_pos: end_of_d + 1,
                        }
                    }
                }
            }
        }

        std::str::from_utf8(&result.into_iter().map(|x| x.unwrap()).collect::<Vec<u8>>())
            .unwrap()
            .to_string()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("IIIDIDDD", "123549876")]
    #[case("DDD", "4321")]
    #[case("IIIDDIDI", "123654879")]
    #[case("IIIDIIDI", "123546879")]
    fn case(#[case] pattern: String, #[case] expected: String) {
        let actual = Solution::smallest_number(pattern);
        assert_eq!(actual, expected);
    }
}
