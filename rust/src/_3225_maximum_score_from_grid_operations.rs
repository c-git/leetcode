//! Solution for https://leetcode.com/problems/maximum-score-from-grid-operations
//! 3225. Maximum Score From Grid Operations

impl Solution {
    /// Translated from https://www.youtube.com/watch?v=0KzdiMKer0Q
    pub fn maximum_score(grid: Vec<Vec<i32>>) -> i64 {
        let n = grid.len();
        if n <= 1 {
            return 0;
        }

        let mut dp = vec![vec![vec![0i64; n + 1]; n + 1]; n];
        let mut prev_max = vec![vec![0i64; n + 1]; n + 1];
        let mut prev_suffix_max = vec![vec![0i64; n + 1]; n + 1];
        let mut col_sum = vec![vec![0i64; n + 1]; n];

        for col in 0..n {
            for row in 1..n {
                col_sum[col][row] = col_sum[col][row - 1] + grid[row - 1][col] as i64;
            }
        }

        for i in 1..n {
            for curr_h in 0..=n {
                for prev_h in 0..=n {
                    if curr_h <= prev_h {
                        let extra_score = col_sum[i][prev_h] - col_sum[i][curr_h];
                        dp[i][curr_h][prev_h] =
                            dp[i][curr_h][prev_h].max(prev_suffix_max[prev_h][0] + extra_score);
                    } else {
                        let extra_score = col_sum[i - 1][curr_h] - col_sum[i - 1][prev_h];
                        dp[i][curr_h][prev_h] = dp[i][curr_h][prev_h]
                            .max(prev_max[prev_h][curr_h] + extra_score)
                            .max(prev_suffix_max[prev_h][curr_h]);
                    }
                }
            }

            for curr_h in 0..=n {
                let mut curr_max = 0;
                for next_h in 0..=n {
                    curr_max = curr_max.max(dp[i][next_h][curr_h]);
                    prev_max[curr_h][next_h] = curr_max;
                }

                let mut curr_suffix_max = 0;
                for next_h in (0..=n).rev() {
                    curr_suffix_max = curr_suffix_max.max(dp[i][next_h][curr_h]);
                    prev_suffix_max[curr_h][next_h] = curr_suffix_max;
                }
            }
        }

        let mut result = 0;
        for prev_h in 0..=n {
            result = result.max(prev_suffix_max[prev_h][0]);
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
    #[case(vec![vec![0,0,0,0,0],vec![0,0,3,0,0],vec![0,1,0,0,0],vec![5,0,0,3,0],vec![0,0,0,0,2]], 11)]
    #[case(vec![vec![10,9,0,0,15],vec![7,1,0,8,0],vec![5,20,0,11,0],vec![0,0,0,1,2],vec![8,12,1,10,3]], 94)]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] expected: i64) {
        let actual = Solution::maximum_score(grid);
        assert_eq!(actual, expected);
    }
}
