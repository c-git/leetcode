//! Solution for https://leetcode.com/problems/find-minimum-time-to-reach-last-room-ii
//! 3342. Find Minimum Time to Reach Last Room II

use std::collections::BinaryHeap;

impl Solution {
    /// Source: https://leetcode.com/problems/find-minimum-time-to-reach-last-room-ii/
    pub fn min_time_to_reach(mut move_time: Vec<Vec<i32>>) -> i32 {
        let (w, h) = (move_time[0].len() - 1, move_time.len() - 1);
        let (d, mut q) = ([0, 1, 0, -1, 0], BinaryHeap::from([(0, 0, 0, 1)]));
        while let Some((t, x, y, dt)) = q.pop() {
            for i in 0..4 {
                let y = (y as i32 + d[i]) as usize;
                let x = (x as i32 + d[i + 1]) as usize;
                if x > w || y > h || move_time[y][x] < 0 {
                    continue;
                }
                let t = dt + (-t).max(move_time[y][x]);
                if x == w && y == h {
                    return t;
                }
                move_time[y][x] = -1;
                q.push((-t, x, y, 3 - dt))
            }
        }
        0
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
    fn case(#[case] move_time: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::min_time_to_reach(move_time);
        assert_eq!(actual, expected);
    }
}
