//! Solution for https://leetcode.com/problems/merge-intervals
//! 56. Merge Intervals

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // Sort intervals so that any interval that starts before the end of the current one being merged ends must show up after
        intervals.sort_unstable();

        // Store current interval being merged at tail of result
        let mut result = vec![intervals
            .first()
            .expect("constraint guarantees at least one interval")
            .clone()];

        // Step though intervals merging or creating a new interval for each seen
        for interval in intervals.into_iter() {
            let last = result.last_mut().expect("starts off non-empty");
            if interval[0] <= last[1] {
                // Overlapping merge
                last[1] = last[1].max(interval[1]);
            } else {
                // Non-overlapping start a new interval
                result.push(interval);
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
    #[case(vec![vec![1,3],vec![2,6],vec![8,10],vec![15,18]], vec![vec![1,6],vec![8,10],vec![15,18]])]
    #[case(vec![vec![1,4],vec![4,5]], vec![vec![1,5]])]
    #[case(vec![vec![1,4],vec![4,5],vec![1,3]], vec![vec![1,5]])]
    fn case(#[case] intervals: Vec<Vec<i32>>, #[case] expected: Vec<Vec<i32>>) {
        let actual = Solution::merge(intervals);
        assert_eq!(actual, expected);
    }
}
