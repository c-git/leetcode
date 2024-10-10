//! Solution for https://leetcode.com/problems/insert-interval
//! 57. Insert Interval

impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::with_capacity(intervals.len() + 1);
        intervals.push(new_interval);
        intervals.sort_by(|x, y| {
            if x[0] == y[0] {
                x[1].cmp(&y[1])
            } else {
                x[0].cmp(&y[0])
            }
        });
        let mut intervals = intervals.into_iter();
        let mut curr_interval = intervals
            .next()
            .expect("we just inserted at least 1 into it");
        for interval in intervals {
            if interval[0] <= curr_interval[1] {
                // Merge
                debug_assert!(
                    interval[0] >= curr_interval[0],
                    "supposed to be sorted by start time"
                );
                curr_interval[1] = curr_interval[1].max(interval[1]);
            } else {
                result.push(curr_interval);
                curr_interval = interval
            }
        }
        result.push(curr_interval);
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
    #[case(vec![vec![1,3],vec![6,9]], vec![2,5], vec![vec![1,5],vec![6,9]])]
    #[case(vec![vec![1,2],vec![3,5],vec![6,7],vec![8,10],vec![12,16]], vec![4,8], vec![vec![1,2],vec![3,10],vec![12,16]])]
    #[case(vec![vec![1,5]], vec![0,0], vec![vec![0,0],vec![1,5]])]
    fn case(
        #[case] intervals: Vec<Vec<i32>>,
        #[case] new_interval: Vec<i32>,
        #[case] expected: Vec<Vec<i32>>,
    ) {
        let actual = Solution::insert(intervals, new_interval);
        assert_eq!(actual, expected);
    }
}
