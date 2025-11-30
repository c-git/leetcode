//! Solution for https://leetcode.com/problems/decode-ways
//! 91. Decode Ways

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        // Solve by keeping track of if we take one or two per how many ways we can continue then return the sum of both at the end
        if s.starts_with('0') {
            // Can never decode strings that start with 0
            return 0;
        }
        if s.len() == 1 {
            // Only one way to decode a string of length 1
            return 1;
        }

        let mut take1 = 1;
        let mut take2 = 0;
        let mut prev_char = None;
        for c in s.chars().rev() {
            match c {
                '0' => {
                    take2 = take1;
                    take1 = 0;
                }
                '1' => {
                    if Some('0') == prev_char {
                        // Take 1 & 2 already correct
                    } else {
                        // Able to do both 1 or 2
                        let temp = take1 + take2;
                        take2 = take1;
                        take1 = temp;
                    }
                }
                '2' => {
                    if let Some(prev) = prev_char
                        && ('1'..='6').contains(&prev)
                    {
                        let temp = take1 + take2;
                        take2 = take1;
                        take1 = temp;
                    } else {
                        take2 = 0;
                    }
                }
                '3'..='9' => {
                    // Only option is to take 1
                    take2 = 0;
                }
                _ => unreachable!("Problem guarantees only digits"),
            }
            if ('1'..='2').contains(&c) && prev_char.is_none() {
                // Take two from here already incorporated in take1 and take2 is not valid by itself anymore
                take2 = 0;
            }

            prev_char = Some(c);
        }

        take1 + take2
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("12", 2)]
    #[case("226", 3)]
    #[case("06", 0)]
    #[case("1", 1)]
    #[case("11", 2)]
    #[case("110", 1)]
    #[case("10", 1)]
    #[case("99", 1)]
    fn case(#[case] s: String, #[case] expected: i32) {
        let actual = Solution::num_decodings(s);
        assert_eq!(actual, expected);
    }
}
