//! Solution for https://leetcode.com/problems/merge-intervals
//! 56. Merge Intervals

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        intervals.sort_unstable();
        let mut it = intervals.into_iter();
        let mut current = it.next().unwrap();
        for interval in it {
            if current[1] >= interval[0] {
                current[1] = current[1].max(interval[1]);
            } else {
                result.push(current);
                current = interval;
            }
        }
        result.push(current);
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
