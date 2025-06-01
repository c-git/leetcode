//! Solution for https://leetcode.com/problems/non-overlapping-intervals
//! 435. Non-overlapping Intervals

impl Solution {
    /// Solution based on https://www.youtube.com/watch?v=2LUQ6tBdGxo
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        intervals.sort_unstable_by_key(|interval| interval[1]);

        let mut last_end_time = intervals[0][1];
        for interval in intervals.into_iter().skip(1) {
            if interval[0] < last_end_time {
                // Overlapping so we need to drop this one
                result += 1;
            } else {
                last_end_time = interval[1];
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
    #[case(vec![vec![1,2],vec![2,3],vec![3,4],vec![1,3]], 1)]
    #[case(vec![vec![1,2],vec![1,2],vec![1,2]], 2)]
    #[case(vec![vec![1,2],vec![2,3]], 0)]
    #[case(vec![vec![0,2],vec![1,3],vec![2,4],vec![3,5],vec![4,6]], 2)]
    fn case(#[case] intervals: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::erase_overlap_intervals(intervals);
        assert_eq!(actual, expected);
    }
}
