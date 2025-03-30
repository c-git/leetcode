//! Solution for https://leetcode.com/problems/reverse-integer
//! 7. Reverse Integer

impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        if x == i32::MIN {
            // Unable to get absolute value so exit early (Reversed value is too big anyway)
            return 0;
        }
        let sign = if x < 0 { -1 } else { 1 };
        let mut result = 0i32;
        x = x.abs();
        while x != 0 {
            let last_digit = x % 10;
            x /= 10;
            result = match result.checked_mul(10) {
                Some(val) => val,
                None => return 0,
            };
            result = match result.checked_add(last_digit) {
                Some(val) => val,
                None => return 0,
            };
        }
        sign * result
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(123, 321)]
    #[case(-123, -321)]
    #[case(120, 21)]
    #[case(i32::MIN, 0)]
    #[case(i32::MAX, 0)]
    #[case(2_000_000_003, 0)]
    fn case(#[case] x: i32, #[case] expected: i32) {
        let actual = Solution::reverse(x);
        assert_eq!(actual, expected);
    }
}
