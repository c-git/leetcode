//! Solution for https://leetcode.com/problems/find-a-peak-element-ii
//! 1901. Find a Peak Element II

struct Bounds {
    top: usize,
    bottom: usize,
    left: usize,
    right: usize,
}
impl Solution {
    pub fn find_peak_grid(mat: Vec<Vec<i32>>) -> Vec<i32> {
        // Based on hints given on leetcode

        let mut bounds = Bounds {
            top: 0,
            bottom: mat.len() - 1,
            left: 0,
            right: mat[0].len() - 1,
        };

        // Decide which way to split to move bigger number into the log in the runtime bound
        let should_split_vertical = bounds.bottom >= bounds.right;

        // Must exit because range considered reduces with each iteration
        loop {
            debug_assert!(
                bounds.left <= bounds.right,
                "Should never be equal as each is 1 further than end"
            );
            debug_assert!(
                bounds.top <= bounds.bottom,
                "Should never be equal as each is 1 further than end"
            );

            let mid = if should_split_vertical {
                bounds.left + bounds.right
            } else {
                bounds.top + bounds.bottom
            } as i32
                / 2;

            let max_mid = Self::get_max(&mat, mid, &bounds, should_split_vertical);
            let max_lower = Self::get_max(&mat, mid - 1, &bounds, should_split_vertical);
            let max_upper = Self::get_max(&mat, mid + 1, &bounds, should_split_vertical);
            if max_mid.0 >= max_lower.0 && max_mid.0 >= max_upper.0 {
                return if should_split_vertical {
                    vec![max_mid.1.unwrap() as i32, mid]
                } else {
                    vec![mid, max_mid.1.unwrap() as i32]
                };
            }

            let is_in_lower = max_lower.0 >= max_upper.0;
            match (is_in_lower, should_split_vertical) {
                (true, true) => bounds.right = mid as usize,
                (true, false) => bounds.bottom = mid as usize,
                (false, true) => bounds.left = mid as usize + 1,
                (false, false) => bounds.top = mid as usize + 1,
            }
        }
    }

    #[allow(clippy::needless_range_loop)]
    /// Returns the (x,y) where x is the max value and y is the index where the value was found
    fn get_max(
        mat: &[Vec<i32>],
        idx_to_check: i32,
        bounds: &Bounds,
        should_split_vertical: bool,
    ) -> (i32, Option<usize>) {
        let mut result = (-1, None);
        if idx_to_check < 0 {
            return result;
        }

        let idx_to_check = idx_to_check as usize;
        if should_split_vertical {
            if idx_to_check >= mat[0].len() {
                return result;
            };
            result = (mat[bounds.top][idx_to_check], Some(bounds.top));
            for row in bounds.top + 1..=bounds.bottom {
                if mat[row][idx_to_check] > result.0 {
                    result = (mat[row][idx_to_check], Some(row));
                }
            }
        } else {
            if idx_to_check >= mat.len() {
                return result;
            };
            result = (mat[idx_to_check][bounds.left], Some(bounds.left));
            for col in bounds.left + 1..=bounds.right {
                if mat[idx_to_check][col] > result.0 {
                    result = (mat[idx_to_check][col], Some(col));
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
    #[case(vec![vec![1,4],vec![3,2]])]
    #[case(vec![vec![10,20,15],vec![21,30,14],vec![7,16,32]])]
    #[case(vec![vec![1,4],vec![4,2]])]
    #[case(vec![vec![10,20,15],vec![21,30,20],vec![20,16,32]])]
    #[case(vec![vec![25,37,23,37,19],vec![45,19,2,43,26],vec![18,1,37,44,50]])]
    fn case(#[case] mat: Vec<Vec<i32>>) {
        // Custom validator written as multiple answers can be the correct answer and
        let input = mat.clone();
        let actual = Solution::find_peak_grid(mat);
        assert_eq!(actual.len(), 2, "Expected exactly 2 values in result");
        let rows = input.len();
        let cols = input[0].len();
        let (row, col) = (actual[0] as usize, actual[1] as usize);
        assert!(row < rows, "Row ({row}) is too large. Length: {rows}");
        assert!(col < cols, "Col ({col}) is too large. Length: {cols}");
        let peak_value = input[row][col];
        if row > 0 {
            assert!(
                input[row - 1][col] < peak_value,
                "Above value not less than peak"
            )
        }
        if col > 0 {
            assert!(
                input[row][col - 1] < peak_value,
                "Left value not less than peak"
            )
        }
        if row < rows - 1 {
            assert!(
                input[row + 1][col] < peak_value,
                "Below value not less than peak"
            )
        }
        if col < cols - 1 {
            assert!(
                input[row][col + 1] < peak_value,
                "Right value not less than peak"
            )
        }
    }
}
