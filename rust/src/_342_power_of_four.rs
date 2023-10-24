//! Solution for https://leetcode.com/problems/power-of-four
//! 342. Power of Four

impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        // Realized could just count the 0's to see if the 1 is in an odd position from
        // https://leetcode.com/problems/power-of-four/solutions/4197440/100-power-of-two-check-masking/
        // Knew I wanted to check where the one was but ignored the fact that directly says how many zeros
        n.count_ones() == 1 && n.trailing_zeros() % 2 == 0
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
    #[case(2, false)]
    #[case(-2147483648, false)]
    fn case(#[case] n: i32, #[case] expected: bool) {
        let actual = Solution::is_power_of_four(n);
        assert_eq!(actual, expected);
    }
}
