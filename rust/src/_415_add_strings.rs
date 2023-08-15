//! Solution for https://leetcode.com/problems/add-strings
//! 415. Add Strings

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let mut iter_a = num1.chars().rev();
        let mut iter_b = num2.chars().rev();
        let mut result = Vec::with_capacity(std::cmp::max(num1.len(), num2.len()) + 1);
        let mut carry = 0;
        loop {
            let val = match (iter_a.next(), iter_b.next()) {
                (None, None) => {
                    if carry > 0 {
                        result.push(carry);
                    }
                    break;
                }
                (Some(x), None) | (None, Some(x)) => x.to_digit(10).unwrap() + carry,
                (Some(x), Some(y)) => x.to_digit(10).unwrap() + y.to_digit(10).unwrap() + carry,
            };
            result.push(val % 10);
            carry = val / 10;
        }
        result
            .iter()
            .rev()
            .map(|&x| char::from_u32(x + b'0' as u32).unwrap())
            .collect()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("11", "123", "134")]
    #[case("456", "77", "533")]
    #[case("0", "0", "0")]
    fn case(#[case] num1: String, #[case] num2: String, #[case] expected: String) {
        let actual = Solution::add_strings(num1, num2);
        assert_eq!(actual, expected);
    }
}
