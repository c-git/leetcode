//! Solution for https://leetcode.com/problems/maximal-rectangle
//! 85. Maximal Rectangle

impl Solution {
    /// Based on https://www.youtube.com/watch?v=dAVF2NpC3j4
    /// Calculate the histogram for each row and solve for max rectangle for that row using subroutine
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let mut result = 0;
        let col_count = matrix[0].len();
        let mut row_histogram = vec![0; col_count];

        // We only need one row so just update histogram in place
        for row in matrix {
            for (col_idx, cell) in row.into_iter().enumerate() {
                row_histogram[col_idx] = if cell == '1' {
                    row_histogram[col_idx] + 1
                } else {
                    0
                };
            }
            result = result.max(Self::max_rectangle_in_histogram(&row_histogram));
        }

        result
    }

    fn max_rectangle_in_histogram(row_histogram: &[i32]) -> i32 {
        let mut result = 0;
        let mut stack: Vec<(usize, i32)> = vec![(0, 0)];
        for (i, height) in row_histogram.iter().enumerate() {
            let mut start_idx = i;
            while let Some((prev_idx, last_val)) = stack.last() {
                if last_val > height {
                    // Last can no longer extend
                    result = result.max(*last_val * (i - prev_idx) as i32);
                    start_idx = *prev_idx;
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push((start_idx, *height));
        }

        // Check rectangles for heights still on the stack
        while let Some((prev_idx, last_val)) = stack.pop() {
            // Last can no longer extend
            result = result.max(last_val * (row_histogram.len() - prev_idx) as i32);
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
    #[case(vec![vec!['1','0','1','0','0'],vec!['1','0','1','1','1'],vec!['1','1','1','1','1'],vec!['1','0','0','1','0']], 6)]
    #[case(vec![vec!['0']], 0)]
    #[case(vec![vec!['1']], 1)]
    #[case(vec![vec!['1','0','1','0','0'],vec!['1','0','1','1','1'],vec!['1','1','1','1','1'],vec!['1','0','1','1','1']], 9)]
    #[case(vec![vec!['0','1'],vec!['1','0']], 1)]
    fn case(#[case] matrix: Vec<Vec<char>>, #[case] expected: i32) {
        let actual = Solution::maximal_rectangle(matrix);
        assert_eq!(actual, expected);
    }
}
