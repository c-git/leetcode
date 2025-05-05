//! Solution for https://leetcode.com/problems/domino-and-tromino-tiling
//! 790. Domino and Tromino Tiling

impl Solution {
    /// Used https://www.youtube.com/watch?v=CecjOo4Zo-g to understand and
    /// https://www.youtube.com/watch?v=ZbEe4lsWW60 for a more efficient solution
    pub fn num_tilings(n: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;

        // Initialize base cases
        let (mut empty, mut up_blocked, mut down_blocked) = (1, 0, 0);
        let mut empty_prev = 1;
        let mut up_blocked_prev;
        let mut down_blocked_prev;
        let mut empty_prev2;

        // Iterate through columns
        for _ in 2..=n {
            // Move current values back
            empty_prev2 = empty_prev;
            (empty_prev, up_blocked_prev, down_blocked_prev) = (empty, up_blocked, down_blocked);

            // Calculate new values
            empty = (empty_prev + empty_prev2 + up_blocked_prev + down_blocked_prev) % MOD;
            up_blocked = (down_blocked_prev + empty_prev2) % MOD;
            down_blocked = (up_blocked_prev + empty_prev2) % MOD;
        }

        empty
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
    fn case(#[case] n: i32, #[case] expected: i32) {
        let actual = Solution::num_tilings(n);
        assert_eq!(actual, expected);
    }
}
