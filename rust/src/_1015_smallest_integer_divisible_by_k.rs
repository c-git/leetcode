//! Solution for https://leetcode.com/problems/smallest-integer-divisible-by-k
//! 1015. Smallest Integer Divisible by K

impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        let mut result = 1;
        let mut seen = vec![false; k as usize];
        let mut remainder = 1;
        remainder %= k;
        seen[remainder as usize] = true;
        while remainder != 0 {
            result += 1;
            remainder *= 10;
            remainder += 1;
            remainder %= k;
            if seen[remainder as usize] {
                // We've hit a loop and it's not possible to ever find a solution
                return -1;
            }
            seen[remainder as usize] = true;
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
    #[case(1, 1)]
    #[case(2, -1)]
    #[case(3, 3)]
    fn case(#[case] k: i32, #[case] expected: i32) {
        let actual = Solution::smallest_repunit_div_by_k(k);
        assert_eq!(actual, expected);
    }
}
