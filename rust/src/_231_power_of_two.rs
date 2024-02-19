//! Solution for https://leetcode.com/problems/power-of-two
//! 231. Power of Two

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        n.count_ones() == 1
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(1, true)]
    #[case(16, true)]
    #[case(3, false)]
    fn case(#[case] n: i32, #[case] expected: bool) {
        let actual = Solution::is_power_of_two(n);
        assert_eq!(actual, expected);
    }
}
