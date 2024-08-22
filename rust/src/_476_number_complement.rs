//! Solution for https://leetcode.com/problems/number-complement
//! 476. Number Complement

use std::ops::BitXor;

impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        // 0 has the all zero bit pattern
        let total_number_of_bits = (0i32).count_zeros();

        // Get was the exponent of 2 that would give 1's starting at the the first 1 in `num`
        let exponent = total_number_of_bits - num.leading_zeros() - 1;

        // Create a number that has that number of 1's starting from the LSB
        let mask = (2 << exponent) - 1;

        // XOR will flip all the bits in `num`
        num.bitxor(mask)
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(5, 2)]
    #[case(1, 0)]
    fn case(#[case] num: i32, #[case] expected: i32) {
        let actual = Solution::find_complement(num);
        assert_eq!(actual, expected);
    }
}
