impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let (is_negative, dividend, divisor) = match (dividend, divisor) {
            (a, b) if a >= 0 && b >= 0 => (false, a as u32, b as u32),
            (a, b) if a >= 0 && b < 0 => (true, a as u32, -(b as i64) as u32),
            (a, b) if a < 0 && b >= 0 => (true, -(a as i64) as u32, b as u32),
            (a, b) if a < 0 && b < 0 => (false, -(a as i64) as u32, -(b as i64) as u32),
            (_, _) => unreachable!(),
        };

        let dividend = dividend.to_string();
        let mut result = String::new();
        let mut carry = 0;
        for c in dividend.chars() {
            carry = Self::mul10(carry) + c.to_digit(10).unwrap();
            let (quotient, remainder) = Self::divide_with_remainder(carry, divisor);
            carry = remainder;
            result.push_str(&quotient.to_string());
        }

        let result: u32 = result.parse().unwrap();
        match (is_negative, result) {
            (false, r) if r >= i32::MAX as u32 => i32::MAX,
            (false, r) => r as i32,
            (true, r) if r > (i32::MAX as u32) => i32::MIN,
            (true, r) => -(r as i32),
        }
    }

    fn divide_with_remainder(mut dividend: u32, divisor: u32) -> (u32, u32) {
        let mut result = 0;
        while dividend >= divisor {
            dividend -= divisor;
            result += 1;
        }
        (result, dividend)
    }

    fn mul10(carry: u32) -> u32 {
        if carry == 0 {
            0
        } else {
            (1..10).fold(carry, |acc, _| acc + carry)
        }
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(10, 3, 3)]
    #[case(7, -3, -2)]
    #[case(-7, 3, -2)]
    #[case(-7, -3, 2)]
    #[case(400, 10, 40)]
    #[case(210, 3, 70)]
    #[case(6009, 3, 2003)]
    #[case(-2147483648, -1, 2147483647)]
    fn case(#[case] dividend: i32, #[case] divisor: i32, #[case] expected: i32) {
        let actual = Solution::divide(dividend, divisor);
        assert_eq!(actual, expected);
    }
}
