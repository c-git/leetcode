//! Solution for https://leetcode.com/problems/minimum-path-sum
//! 64. Minimum Path Sum

impl Solution {
    /// Replace each cell with the min value to get to that cell
    /// For each cell we can only get there from the left of the top so just
    /// update to the cell value plus the min of those two
    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        // For top row we can only get there from the left
        for col in 1..grid[0].len() {
            grid[0][col] += grid[0][col - 1];
        }

        for row in 1..grid.len() {
            grid[row][0] += grid[row - 1][0]; // We can only get to the left most cell from up
            for col in 1..grid[row].len() {
                grid[row][col] += grid[row - 1][col].min(grid[row][col - 1]);
            }
        }

        *grid.last().unwrap().last().unwrap()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,3,1],vec![1,5,1],vec![4,2,1]], 7)]
    #[case(vec![vec![1,2,3],vec![4,5,6]], 12)]
    #[case(vec![vec![1,2],vec![1,1]], 3)]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::min_path_sum(grid);
        assert_eq!(actual, expected);
    }
}
