//! Solution for https://leetcode.com/problems/find-minimum-time-to-reach-last-room-ii
//! 3342. Find Minimum Time to Reach Last Room II

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        let row_count = move_time.len();
        let col_count = move_time[0].len();
        let mut heap = BinaryHeap::new();
        heap.push(Reverse((0, 0, 0, true)));
        let mut visited = vec![vec![false; col_count]; row_count];
        while let Some(Reverse((time, row, col, was_two_sec_move))) = heap.pop() {
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
                add_room(
                    &move_time,
                    &mut heap,
                    &visited,
                    time,
                    row - 1,
                    col,
                    was_two_sec_move,
                );
            }

            // Check Down
            if row < row_count - 1 {
                add_room(
                    &move_time,
                    &mut heap,
                    &visited,
                    time,
                    row + 1,
                    col,
                    was_two_sec_move,
                );
            }

            // Check Left
            if col > 0 {
                add_room(
                    &move_time,
                    &mut heap,
                    &visited,
                    time,
                    row,
                    col - 1,
                    was_two_sec_move,
                );
            }

            // Check Right
            if col < col_count - 1 {
                add_room(
                    &move_time,
                    &mut heap,
                    &visited,
                    time,
                    row,
                    col + 1,
                    was_two_sec_move,
                );
            }
        }
        unreachable!("we must always be able to get to the destination")
    }
}

#[inline]
fn add_room(
    move_time: &[Vec<i32>],
    heap: &mut BinaryHeap<Reverse<(i32, usize, usize, bool)>>,
    visited: &[Vec<bool>],
    time: i32,
    row: usize,
    col: usize,
    was_two_sec_move: bool,
) {
    if !visited[row][col] {
        let min_start_time = time.max(move_time[row][col]);
        heap.push(Reverse((
            min_start_time + if was_two_sec_move { 1 } else { 2 },
            row,
            col,
            !was_two_sec_move,
        )));
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![0,4],vec![4,4]], 7)]
    #[case(vec![vec![0,0,0,0],vec![0,0,0,0]], 6)]
    #[case(vec![vec![0,1],vec![1,2]], 4)]
    #[case(vec![vec![0,58],vec![27,69]], 71)]
    fn case(#[case] move_time: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::min_time_to_reach(move_time);
        assert_eq!(actual, expected);
    }
}
