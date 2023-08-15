//! Solution for https://leetcode.com/problems/add-binary
//! 67. Add Binary

use std::cmp::max;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut iter_a = a.chars().rev();
        let mut iter_b = b.chars().rev();
        let mut result = Vec::with_capacity(max(a.len(), b.len()) + 1);
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
            match val {
                0 => {
                    result.push(0);
                    // carry = 0; // Not needed as carry must already be 0 otherwise val wouldn't be 0
                }
                1 => {
                    result.push(1);
                    carry = 0;
                }
                2 => {
                    result.push(0);
                    carry = 1;
                }
                3 => {
                    result.push(1);
                    carry = 1;
                }
                _ => unreachable!("Values should only be binary"),
            }
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
    #[case("11", "1", "100")]
    #[case("1010", "1011", "10101")]
    fn case(#[case] a: String, #[case] b: String, #[case] expected: String) {
        let actual = Solution::add_binary(a, b);
        assert_eq!(actual, expected);
    }
}
