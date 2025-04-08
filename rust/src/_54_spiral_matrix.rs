//! Solution for https://leetcode.com/problems/spiral-matrix
//! 54. Spiral Matrix

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let row_count = matrix.len();
        let col_count = matrix[0].len();
        let mut result = Vec::with_capacity(row_count * col_count);
        let mut bound_left = 0;
        let mut bound_right = col_count - 1;
        let mut bound_top = 1;
        let mut bound_bottom = row_count - 1;
        let mut is_row_changing = false;
        let mut is_increasing = true;
        let mut curr_row = 0;
        let mut curr_col = 0;
        while (bound_left <= bound_right && !is_row_changing)
            || (bound_top <= bound_bottom && is_row_changing)
        {
            result.push(matrix[curr_row][curr_col]);
            match (is_row_changing, is_increasing) {
                (true, true) => {
                    // Going Down
                    if curr_row < bound_bottom {
                        curr_row += 1;
                    } else {
                        is_row_changing = false;
                        is_increasing = false;
                        curr_col -= 1;
                        bound_bottom -= 1;
                    }
                }
                (true, false) => {
                    // Going Up
                    if curr_row > bound_top {
                        curr_row -= 1;
                    } else {
                        is_row_changing = false;
                        is_increasing = true;
                        curr_col += 1;
                        bound_top += 1;
                    }
                }
                (false, true) => {
                    // Going Right
                    if curr_col < bound_right {
                        curr_col += 1;
                    } else {
                        is_row_changing = true;
                        curr_row += 1;
                        bound_right -= 1;
                    }
                }
                (false, false) => {
                    // Going Left
                    if curr_col > bound_left {
                        curr_col -= 1;
                    } else {
                        is_row_changing = true;
                        curr_row -= 1;
                        bound_left += 1;
                    }
                }
            }
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
    #[case(vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]], vec![1,2,3,6,9,8,7,4,5])]
    #[case(vec![vec![1,2,3,4],vec![5,6,7,8],vec![9,10,11,12]], vec![1,2,3,4,8,12,11,10,9,5,6,7])]
    fn case(#[case] matrix: Vec<Vec<i32>>, #[case] expected: Vec<i32>) {
        let actual = Solution::spiral_order(matrix);
        assert_eq!(actual, expected);
    }
}
