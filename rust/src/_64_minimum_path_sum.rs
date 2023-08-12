//! Solution for https://leetcode.com/problems/minimum-path-sum
//! 64. Minimum Path Sum

use std::cmp::min;

impl Solution {
    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        for col in 1..grid[0].len() {
            grid[0][col] += grid[0][col - 1];
        }

        for row in 1..grid.len() {
            grid[row][0] += grid[row - 1][0];
            for col in 1..grid[row].len() {
                grid[row][col] += min(grid[row - 1][col], grid[row][col - 1]);
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
