//! Solution for https://leetcode.com/problems/greatest-common-divisor-of-strings
//! 1071. Greatest Common Divisor of Strings

use std::mem::swap;

impl Solution {
    pub fn gcd_of_strings(mut str1: String, mut str2: String) -> String {
        // Ensure str1 is the shorter of the two
        if str1.len() > str2.len() {
            swap(&mut str1, &mut str2);
        }
        let mut scratch_space = String::with_capacity(str1.len());
        let mut divisor = str1.clone();
        while !Self::divisible(&str1, &divisor, &mut scratch_space)
            || !Self::divisible(&str2, &divisor, &mut scratch_space)
        {
            divisor.pop(); // Shorten divisor until it works
        }
        divisor
    }

    fn divisible(s: &str, divisor: &str, scratch_space: &mut String) -> bool {
        debug_assert!(
            divisor.len() <= s.len(),
            "Expected divisor length '{} - ({})' to be at least as short as the length of s '{} - ({})' but this did not hold",
            divisor, divisor.len(), s, s.len()
        );

        if divisor.is_empty() {
            // Nothing divides everything (As per example, even thought it doesn't quite make sense)
            return true;
        }
        if s.len() % divisor.len() != 0 {
            // Will never be equal as it is not a multiple of it's length
            return false;
        }

        scratch_space.clear();
        scratch_space.push_str(divisor);
        while scratch_space.len() <= s.len() {
            if scratch_space == s {
                return true;
            }
            scratch_space.push_str(divisor);
        }
        false
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("ABCABC", "ABC", "ABC")]
    #[case("ABABAB", "ABAB", "AB")]
    #[case("LEET", "CODE", "")]
    fn case(#[case] str1: String, #[case] str2: String, #[case] expected: String) {
        let actual = Solution::gcd_of_strings(str1, str2);
        assert_eq!(actual, expected);
    }
}
