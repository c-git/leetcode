//! Solution for https://leetcode.com/problems/decode-ways
//! 91. Decode Ways

impl Solution {
    /// Used https://www.youtube.com/watch?v=6aEyTjOwlJU to debug
    pub fn num_decodings(s: String) -> i32 {
        // Solve by keeping track of if we take one or two per how many ways we can continue then return the sum of both at the end
        let mut continue_at_next = 1; // Number of ways to decode starting at next digit
        let mut skip_next = 0; // Number of ways to decode if we skip the next digit
        let mut prev_char = None;
        for c in s.chars().rev() {
            (continue_at_next, skip_next) = match c {
                '0' => {
                    // Keep skip as an option but 0 options starting from here
                    (0, continue_at_next)
                }
                '1' => {
                    // We can do both take an skip
                    (continue_at_next + skip_next, continue_at_next)
                }
                '2' => {
                    if let Some(prev) = prev_char
                        && ('0'..='6').contains(&prev)
                    {
                        (continue_at_next + skip_next, continue_at_next)
                    } else {
                        (continue_at_next, continue_at_next)
                    }
                }
                '3'..='9' => (continue_at_next, continue_at_next),
                _ => unreachable!("Problem guarantees only digits"),
            };

            prev_char = Some(c);
        }

        continue_at_next
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
    #[case("1201234", 3)]
    fn case(#[case] s: String, #[case] expected: i32) {
        let actual = Solution::num_decodings(s);
        assert_eq!(actual, expected);
    }
}
