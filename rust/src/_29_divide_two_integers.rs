impl Solution {
    pub fn divide(mut dividend: i32, mut divisor: i32) -> i32 {
        let is_negative = match (dividend, divisor) {
            (a, b) if a >= 0 && b >= 0 => false,
            (a, b) if a >= 0 && b < 0 => {
                divisor = -divisor;
                true
            }
            (a, b) if a < 0 && b >= 0 => {
                dividend = -dividend;
                true
            }
            (a, b) if a < 0 && b < 0 => {
                dividend = -dividend;
                divisor = -divisor;
                false
            }
            (_, _) => unreachable!(),
        };
        debug_assert!(dividend >= 0 && divisor >= 0);

        let dividend = dividend.to_string();
        let mut result = String::new();
        let mut carry = 0;
        let divisor = divisor as u32;
        for c in dividend.chars() {
            carry = Self::mul10(carry) + c.to_digit(10).unwrap();
            let (quotient, remainder) = Self::divide_with_remainder(carry, divisor);
            carry = remainder;
            result.push_str(&quotient.to_string());
        }

        let result: i32 = result.parse().unwrap();
        if is_negative {
            -result
        } else {
            result
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
        dbg!(if carry == 0 {
            0
        } else {
            (1..10).fold(carry, |acc, _| acc + carry)
        })
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
    fn case(#[case] dividend: i32, #[case] divisor: i32, #[case] expected: i32) {
        let actual = Solution::divide(dividend, divisor);
        assert_eq!(actual, expected);
    }
}
