//! Solution for https://leetcode.com/problems/01-matrix
//! 542. 01 Matrix

use std::collections::VecDeque;

impl Solution {
    pub fn update_matrix(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut seen = vec![vec![false; mat[0].len()]; mat.len()];

        // Stores tuples of (row, col, distance)
        let mut queue = VecDeque::new();

        // mark 0 nodes
        let rows = mat.len();
        let cols = mat[0].len();
        for row in 0..rows {
            for col in 0..cols {
                if mat[row][col] == 0 {
                    seen[row][col] = true;
                    queue.push_back((row, col, 0));
                }
            }
        }

        while let Some((row, col, distance)) = queue.pop_front() {
            mat[row][col] = distance;

            // Up
            if row > 0 {
                check_neighbour(&mut seen, &mut queue, row - 1, col, distance);
            }

            // Down
            if row + 1 < rows {
                check_neighbour(&mut seen, &mut queue, row + 1, col, distance);
            }

            // Left
            if col > 0 {
                check_neighbour(&mut seen, &mut queue, row, col - 1, distance);
            }

            // Right
            if col + 1 < cols {
                check_neighbour(&mut seen, &mut queue, row, col + 1, distance);
            }
        }
        mat
    }
}

fn check_neighbour(
    seen: &mut [Vec<bool>],
    queue: &mut VecDeque<(usize, usize, i32)>,
    row: usize,
    col: usize,
    distance: i32,
) {
    if !seen[row][col] {
        seen[row][col] = true;
        queue.push_back((row, col, distance + 1));
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![0,0,0],vec![0,1,0],vec![0,0,0]], vec![vec![0,0,0],vec![0,1,0],vec![0,0,0]])]
    #[case(vec![vec![0,0,0],vec![0,1,0],vec![1,1,1]], vec![vec![0,0,0],vec![0,1,0],vec![1,2,1]])]
    fn case(#[case] mat: Vec<Vec<i32>>, #[case] expected: Vec<Vec<i32>>) {
        let actual = Solution::update_matrix(mat);
        assert_eq!(actual, expected);
    }
}
