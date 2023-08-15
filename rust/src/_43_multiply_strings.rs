//! Solution for https://leetcode.com/problems/multiply-strings
//! 43. Multiply Strings

use std::cmp::max;

type LeNumber = Vec<u8>; // Little Endian Number (Each u8 represents a single digit and must be 0-9)

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let num1: LeNumber = num1.as_bytes().iter().rev().map(|x| x - b'0').collect();
        let num2: LeNumber = num2.as_bytes().iter().rev().map(|x| x - b'0').collect();
        let mut result: LeNumber = vec![0];

        for (i, digit) in num2.iter().enumerate() {
            let intermediate = Self::multiply_digit(&num1, digit, i);
            result = Self::add(&intermediate, &result);
        }

        result
            .iter()
            .rev()
            .map(|&x| char::from_u32((x + b'0') as u32).expect("Should be single digit"))
            .collect()
    }

    fn add(num1: &LeNumber, num2: &LeNumber) -> LeNumber {
        let mut result = Vec::with_capacity(max(num1.len(), num2.len()) + 1);
        let mut iter1 = num1.iter();
        let mut iter2 = num2.iter();
        let mut carry = 0;
        loop {
            match (iter1.next(), iter2.next()) {
                (None, None) => {
                    if carry > 0 {
                        result.push(carry);
                    }
                    break;
                }
                (Some(x), None) | (None, Some(x)) => {
                    let val = x + carry;
                    result.push(val % 10);
                    carry = val / 10;
                }
                (Some(x), Some(y)) => {
                    let val = x + y + carry;
                    result.push(val % 10);
                    carry = val / 10;
                }
            }
        }
        result
    }

    fn multiply_digit(num1: &LeNumber, digit: &u8, left_shift: usize) -> LeNumber {
        let mut result = vec![0; left_shift];
        let mut carry = 0;
        for num in num1 {
            let val = num * digit + carry;
            result.push(val % 10);
            carry = val / 10;
        }
        if carry > 0 {
            result.push(carry);
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
    #[case("2", "3", "6")]
    #[case("123", "456", "56088")]
    fn case(#[case] num1: String, #[case] num2: String, #[case] expected: String) {
        let actual = Solution::multiply(num1, num2);
        assert_eq!(actual, expected);
    }
}
