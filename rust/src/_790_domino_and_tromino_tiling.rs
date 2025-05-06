//! Solution for https://leetcode.com/problems/domino-and-tromino-tiling
//! 790. Domino and Tromino Tiling

impl Solution {
    /// Used https://www.youtube.com/watch?v=CecjOo4Zo-g to understand and
    /// https://www.youtube.com/watch?v=ZbEe4lsWW60 for a more efficient solution
    pub fn num_tilings(n: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;

        // Initialize base cases
        let (mut empty, mut half_blocked) = (1, 0);
        let mut empty_prev = 1;
        let mut half_blocked_prev;
        let mut empty_prev2;

        // Iterate through columns
        for _ in 2..=n {
            // Move current values back
            empty_prev2 = empty_prev;
            (empty_prev, half_blocked_prev) = (empty, half_blocked);

            // Calculate new values
            empty = (empty_prev + empty_prev2 + half_blocked_prev * 2) % MOD;
            half_blocked = (half_blocked_prev + empty_prev2) % MOD;
        }

        empty as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(3, 5)]
    #[case(1, 1)]
    #[case(30, 312342182)]
    #[case(1000, 979232805)]
    fn case(#[case] n: i32, #[case] expected: i32) {
        let actual = Solution::num_tilings(n);
        assert_eq!(actual, expected);
    }
}
