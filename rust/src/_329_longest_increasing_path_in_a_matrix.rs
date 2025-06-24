//! Solution for https://leetcode.com/problems/longest-increasing-path-in-a-matrix
//! 329. Longest Increasing Path in a Matrix

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        // Stores longest sequence starting at a given cell if explored
        let mut dp: Vec<Vec<Option<i32>>> = vec![vec![None; matrix[0].len()]; matrix.len()];
        for row in 0..matrix.len() {
            for col in 0..matrix[0].len() {
                if dp[row][col].is_none() {
                    // Populate any unexplored cells
                    Self::explore(row, col, &matrix, &mut dp);
                }

                // Always update result so that we check all cells
                result = result.max(dp[row][col].expect("should have been filled during explore"));
            }
        }
        result
    }

    /// Returns value for cell Doesn't worry about loops as we are looking for strictly increasing and thus we cannot reverse
    fn explore(row: usize, col: usize, matrix: &[Vec<i32>], dp: &mut [Vec<Option<i32>>]) -> i32 {
        if let Some(x) = dp[row][col] {
            return x; // Already done no work needed
        }

        let mut best_neighbour = 0;

        // Check Up
        if row > 0 && matrix[row][col] < matrix[row - 1][col] {
            best_neighbour = best_neighbour.max(Self::explore(row - 1, col, matrix, dp));
        }

        // Check Down
        if row < matrix.len() - 1 && matrix[row][col] < matrix[row + 1][col] {
            best_neighbour = best_neighbour.max(Self::explore(row + 1, col, matrix, dp));
        }

        // Check Left
        if col > 0 && matrix[row][col] < matrix[row][col - 1] {
            best_neighbour = best_neighbour.max(Self::explore(row, col - 1, matrix, dp));
        }

        // Check Right
        if col < matrix[0].len() - 1 && matrix[row][col] < matrix[row][col + 1] {
            best_neighbour = best_neighbour.max(Self::explore(row, col + 1, matrix, dp));
        }

        let result = best_neighbour + 1;
        dp[row][col] = Some(result);
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
    #[case(vec![vec![9,9,4],vec![6,6,8],vec![2,1,1]], 4)]
    #[case(vec![vec![3,4,5],vec![3,2,6],vec![2,2,1]], 4)]
    #[case(vec![vec![1]], 1)]
    fn case(#[case] matrix: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::longest_increasing_path(matrix);
        assert_eq!(actual, expected);
    }
}
