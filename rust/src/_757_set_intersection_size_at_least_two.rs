//! Solution for https://leetcode.com/problems/set-intersection-size-at-least-two
//! 757. Set Intersection Size At Least Two

use std::cmp::Reverse;

impl Solution {
    /// Based on https://www.youtube.com/watch?v=la4OkK3pY7o
    pub fn intersection_size_two(mut intervals: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        intervals.sort_unstable_by_key(|x| (x[1], Reverse(x[0])));

        let mut p1 = -1;
        let mut p2 = -1;

        for (left, right) in intervals.into_iter().map(|x| (x[0], x[1])) {
            if p2 < left {
                result += 2;
                p1 = right - 1;
                p2 = right;
            } else if p1 < left {
                result += 1;
                if p2 == right {
                    p1 = right - 1;
                } else {
                    p1 = p2;
                    p2 = right;
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
    #[case(vec![vec![1,3],vec![3,7],vec![8,9]], 5)]
    #[case(vec![vec![1,3],vec![1,4],vec![2,5],vec![3,5]], 3)]
    #[case(vec![vec![1,2],vec![2,3],vec![2,4],vec![4,5]], 5)]
    fn case(#[case] intervals: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::intersection_size_two(intervals);
        assert_eq!(actual, expected);
    }
}
