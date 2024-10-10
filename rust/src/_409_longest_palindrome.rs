//! Solution for https://leetcode.com/problems/longest-palindrome
//! 409. Longest Palindrome

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        // ASSUMPTION: as stated in the question that only English Upper and Lower case letters will be used
        // 'z'-'A'=? -> 122-65=57
        // Using a u64 instead of an array as all we need is true and false
        let mut result = 0;

        // The 0th bit represents 'A' and other characters are offset by their ascii value.
        // If the bit is 1 then we have one we can pair with
        let mut available = 0u64;
        for b in s.bytes() {
            let bit_pos = b - b'A';
            let mask = 1 << bit_pos;
            if (available & mask) != 0 {
                // The bit is at index `bit_pos` is 1
                result += 2;
            }

            // Flip the bit we either used it or we need to make it available for next time
            available ^= mask;
        }
        if available != 0 {
            // We have at least one more character that we can use in the middle to extend
            result += 1;
        }
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
    #[case("abccccdd", 7)]
    #[case("a", 1)]
    fn case(#[case] s: String, #[case] expected: i32) {
        let actual = Solution::longest_palindrome(s);
        assert_eq!(actual, expected);
    }
}
