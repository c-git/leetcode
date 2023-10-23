//! Solution for https://leetcode.com/problems/power-of-four
//! 342. Power of Four

impl Solution {
    pub fn is_power_of_four(mut n: i32) -> bool {
        if n.count_ones() != 1 {
            return false;
        }
        while n >= 4 {
            n >>= 2;
        }
        n == 1
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(16, true)]
    #[case(5, false)]
    #[case(1, true)]
    #[case(4, true)]
    fn case(#[case] n: i32, #[case] expected: bool) {
        let actual = Solution::is_power_of_four(n);
        assert_eq!(actual, expected);
    }
}
