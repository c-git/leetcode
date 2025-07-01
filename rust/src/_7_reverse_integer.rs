//! Solution for https://leetcode.com/problems/reverse-integer
//! 7. Reverse Integer

impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        if x == i32::MIN {
            return 0;
        }

        let is_negative = x.is_negative();
        x = x.abs();
        let mut result = 0i32;
        while x > 0 {
            result = match result.checked_mul(10) {
                Some(next) => next,
                None => return 0, // Invalid value
            };
            result += x % 10; // only top digit can cause overflow but it's a 2 which cannot
            x /= 10;
        }

        if is_negative {
            -result
        } else {
            result
        }
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
