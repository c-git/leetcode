//! Solution for https://leetcode.com/problems/maximum-path-score-in-a-grid
//! 3742. Maximum Path Score in a Grid

impl Solution {
    /// Didn't figure out we needed a 3rd dimension on DP. So looked it up in https://www.youtube.com/watch?v=ptB843R2pMI
    pub fn max_path_score(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let row_count = grid.len();
        let col_count = grid[0].len();
        let k = k as usize;
        let empty = vec![vec![None; k + 1]; col_count];
        let mut dp_curr_row = empty.clone();
        let mut dp_next_row = empty.clone();
        dp_next_row[0][0] = Some(0);

        for row in 0..row_count {
            // Switch next row to current and clear for next iteration
            std::mem::swap(&mut dp_curr_row, &mut dp_next_row);
            dp_next_row = empty.clone();
            for col in 0..col_count {
                for cost in 0..=k {
                    if dp_curr_row[col][cost].is_none() {
                        continue;
                    }

                    if row + 1 < row_count {
                        let val = grid[row + 1][col];
                        let this_cost = if val == 0 { 0 } else { 1 };
                        if cost + this_cost <= k {
                            dp_next_row[col][cost + this_cost] = dp_next_row[col][cost + this_cost]
                                .max(Some(dp_curr_row[col][cost].unwrap() + val));
                        }
                    }

                    if col + 1 < col_count {
                        let val = grid[row][col + 1];
                        let this_cost = if val == 0 { 0 } else { 1 };
                        if cost + this_cost <= k {
                            dp_curr_row[col + 1][cost + this_cost] = dp_curr_row[col + 1]
                                [cost + this_cost]
                                .max(Some(dp_curr_row[col][cost].unwrap() + val));
                        }
                    }
                }
            }
        }

        (0..=k)
            .map(|cost| dp_curr_row[col_count - 1][cost])
            .max()
            .expect("the max should be over at least one value")
            .unwrap_or(-1)
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![0, 1],vec![2, 0]], 1, 2)]
    #[case(vec![vec![0, 1],vec![1, 2]], 1, -1)]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::max_path_score(grid, k);
        assert_eq!(actual, expected);
    }
}
