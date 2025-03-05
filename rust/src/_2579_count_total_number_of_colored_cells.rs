//! Solution for https://leetcode.com/problems/count-total-number-of-colored-cells
//! 2579. Count Total Number of Colored Cells

impl Solution {
    pub fn colored_cells(n: i32) -> i64 {
        // Each timestamp the number of cells that it increases by is 4 more
        // Values 1 -> 5 -> 13 -> 25 -> 41 -> 61
        // Diffs       4 ->  8 -> 12 -> 16 -> 20

        // Sum of arithmetic series plus 1
        // Source: https://en.wikipedia.org/wiki/Arithmetic_progression
        let n = n as i64;
        ((n - 1) * (n * 4)) / 2 + 1
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
    #[case(2, 5)]
    #[case(3, 13)]
    #[case(4, 25)]
    #[case(5, 41)]
    #[case(6, 61)]
    #[case(7, 85)]
    #[case(8, 113)]
    #[case(9, 145)]
    #[case(10, 181)]
    #[case(100, 19801)]
    #[case(1000, 1998001)]
    fn case(#[case] n: i32, #[case] expected: i64) {
        let actual = Solution::colored_cells(n);
        assert_eq!(actual, expected);
    }
}
