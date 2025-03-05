//! Solution for https://leetcode.com/problems/check-if-number-is-a-sum-of-powers-of-three
//! 1780. Check if Number is a Sum of Powers of Three

const POWERS_OF_3: [i32; 15] = [
    4782969, 1594323, 531441, 177147, 59049, 19683, 6561, 2187, 729, 243, 81, 27, 9, 3, 1,
];

impl Solution {
    pub fn check_powers_of_three(mut n: i32) -> bool {
        // Given the small size of the input maxing out at 10^7 just hard code the
        // powers of 3
        for pow in POWERS_OF_3 {
            if n >= pow {
                n -= pow;
            }
        }
        n == 0
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(12, true)]
    #[case(91, true)]
    #[case(21, false)]
    fn case(#[case] n: i32, #[case] expected: bool) {
        let actual = Solution::check_powers_of_three(n);
        assert_eq!(actual, expected);
    }
}
