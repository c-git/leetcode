//! Solution for https://leetcode.com/problems/01-matrix
//! 542. 01 Matrix

use std::collections::VecDeque;

impl Solution {
    pub fn update_matrix(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // Change all non-zero values to -1 to indicate they are not done then do BFS to
        // update the values after adding all the 0 positions to the queue

        // Stores tuples of (row, col)
        let mut queue = VecDeque::new();

        // Add 0 nodes to queue
        for (row_idx, row) in mat.iter_mut().enumerate() {
            for (col_idx, cell) in row.iter_mut().enumerate() {
                if *cell == 0 {
                    queue.push_back((row_idx, col_idx));
                } else {
                    // Mark as not done
                    *cell = -1;
                }
            }
        }

        let rows_count = mat.len();
        let cols_count = mat[0].len();
        while let Some((row, col)) = queue.pop_front() {
            let new_distance = mat[row][col] + 1;

            // Up
            if row > 0 {
                check_neighbour(&mut mat, &mut queue, row - 1, col, new_distance);
            }

            // Down
            if row + 1 < rows_count {
                check_neighbour(&mut mat, &mut queue, row + 1, col, new_distance);
            }

            // Left
            if col > 0 {
                check_neighbour(&mut mat, &mut queue, row, col - 1, new_distance);
            }

            // Right
            if col + 1 < cols_count {
                check_neighbour(&mut mat, &mut queue, row, col + 1, new_distance);
            }
        }
        mat
    }
}

fn check_neighbour(
    mat: &mut [Vec<i32>],
    queue: &mut VecDeque<(usize, usize)>,
    row: usize,
    col: usize,
    new_distance: i32,
) {
    if mat[row][col] < 0 {
        mat[row][col] = new_distance; // Update distance as we could not have reached it earlier
        queue.push_back((row, col)); // Add to queue to check neighbours
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
