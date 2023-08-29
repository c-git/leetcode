//! Solution for https://leetcode.com/problems/merge-intervals
//! 56. Merge Intervals

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        intervals.sort_unstable();
        let mut curr_start = intervals[0][0];
        let mut curr_end = intervals[0][1];
        for interval in intervals.into_iter().skip(1) {
            let start = interval[0];
            let end = interval[1];
            if start <= curr_end {
                curr_end = curr_end.max(end);
            } else {
                result.push(vec![curr_start, curr_end]);
                curr_start = start;
                curr_end = end;
            }
        }
        result.push(vec![curr_start, curr_end]);
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
    #[case(vec![vec![1,3],vec![2,6],vec![8,10],vec![15,18]], vec![vec![1,6],vec![8,10],vec![15,18]])]
    #[case(vec![vec![1,4],vec![4,5]], vec![vec![1,5]])]
    fn case(#[case] intervals: Vec<Vec<i32>>, #[case] expected: Vec<Vec<i32>>) {
        let actual = Solution::merge(intervals);
        assert_eq!(actual, expected);
    }
}
