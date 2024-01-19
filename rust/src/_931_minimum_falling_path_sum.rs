//! Solution for https://leetcode.com/problems/minimum-falling-path-sum
//! 931. Minimum Falling Path Sum

use std::mem;

impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();
        let mut matrix_iter = matrix.into_iter();
        let mut prev_row = vec![0; n];
        let mut curr_row = matrix_iter
            .next()
            .expect("Constraints guarantee at least 1 row");
        for row in matrix_iter {
            mem::swap(&mut prev_row, &mut curr_row);

            for i in 0..n {
                let val = row[i];
                curr_row[i] = prev_row[i] + val;
                if i > 0 {
                    // Come from above on the left
                    curr_row[i] = curr_row[i].min(prev_row[i - 1] + val);
                }
                if let Some(prev_val) = prev_row.get(i + 1) {
                    // Come from above on the right
                    curr_row[i] = curr_row[i].min(prev_val + val);
                }
            }
        }
        *curr_row
            .iter()
            .min()
            .expect("Constraints guarantee at least 1 column")
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![2,1,3],vec![6,5,4],vec![7,8,9]], 13)]
    #[case(vec![vec![-19,57],vec![-40,-5]], -59)]
    fn case(#[case] matrix: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::min_falling_path_sum(matrix);
        assert_eq!(actual, expected);
    }
}
