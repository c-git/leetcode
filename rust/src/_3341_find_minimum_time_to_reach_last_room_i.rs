//! Solution for https://leetcode.com/problems/find-minimum-time-to-reach-last-room-i
//! 3341. Find Minimum Time to Reach Last Room I

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        let row_count = move_time.len();
        let col_count = move_time[0].len();
        let mut heap = BinaryHeap::new();
        heap.push(Reverse((0, 0, 0)));
        let mut visited = vec![vec![false; col_count]; row_count];
        while let Some(Reverse((time, row, col))) = heap.pop() {
            if visited[row][col] {
                // Already visited we found a faster way
                continue;
            }
            if row == row_count - 1 && col == col_count - 1 {
                // Target found
                return time;
            }
            visited[row][col] = true;

            // Check Up
            if row > 0 {
                add_room(&move_time, &mut heap, &visited, time, row - 1, col);
            }

            // Check Down
            if row < row_count - 1 {
                add_room(&move_time, &mut heap, &visited, time, row + 1, col);
            }

            // Check Left
            if col > 0 {
                add_room(&move_time, &mut heap, &visited, time, row, col - 1);
            }

            // Check Right
            if col < col_count - 1 {
                add_room(&move_time, &mut heap, &visited, time, row, col + 1);
            }
        }
        unreachable!("we must always be able to get to the destination")
    }
}

#[inline]
fn add_room(
    move_time: &[Vec<i32>],
    heap: &mut BinaryHeap<Reverse<(i32, usize, usize)>>,
    visited: &[Vec<bool>],
    time: i32,
    row: usize,
    col: usize,
) {
    if !visited[row][col] {
        let min_start_time = time.max(move_time[row][col]);
        heap.push(Reverse((min_start_time + 1, row, col)));
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![0,4],vec![4,4]], 6)]
    #[case(vec![vec![0,0,0],vec![0,0,0]], 3)]
    #[case(vec![vec![0,1],vec![1,2]], 3)]
    fn case(#[case] move_time: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::min_time_to_reach(move_time);
        assert_eq!(actual, expected);
    }
}
