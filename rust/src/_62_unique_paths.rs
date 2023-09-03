//! Solution for https://leetcode.com/problems/unique-paths
//! 62. Unique Paths

use std::mem;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut dp = vec![1; n as usize]; // There is only one way to get to each cell on the first row
        let mut prev = dp.clone(); // Copy dp so we can modify it
        for _ in 1..m {
            let mut last = dp[0];
            // Update value for each position for each row after the first
            for (i, cell) in dp.iter_mut().enumerate().skip(1) {
                // Skipping the first cell update all cells based on the one above and to the left
                *cell = prev[i] + last;
                last = *cell;
            }
            mem::swap(&mut dp, &mut prev);
        }
        prev[prev.len() - 1]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(3, 7, 28)]
    #[case(3, 2, 3)]
    fn case(#[case] m: i32, #[case] n: i32, #[case] expected: i32) {
        let actual = Solution::unique_paths(m, n);
        assert_eq!(actual, expected);
    }
}
