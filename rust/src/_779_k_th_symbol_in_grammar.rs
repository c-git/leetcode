//! Solution for https://leetcode.com/problems/k-th-symbol-in-grammar
//! 779. K-th Symbol in Grammar

impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        // Based on Beixuan's solution
        if n == 1 {
            debug_assert!(k == 1);
            return 0;
        }
        let parent = Self::kth_grammar(n - 1, (k + 1) / 2);
        if k % 2 == 1 {
            parent
        } else {
            1 - parent
        }
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(1, 1, 0)]
    #[case(2, 1, 0)]
    #[case(2, 2, 1)]
    #[case(30, 536870912, 1)]
    fn case(#[case] n: i32, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::kth_grammar(n, k);
        assert_eq!(actual, expected);
    }
}
