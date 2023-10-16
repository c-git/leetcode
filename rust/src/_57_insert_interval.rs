//! Solution for https://leetcode.com/problems/insert-interval
//! 57. Insert Interval

enum MergeStatus {
    Before,
    During([i32; 2]),
    After,
}

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::with_capacity(intervals.len() + 1);
        let mut merge_status = MergeStatus::Before;
        for interval in intervals {
            match merge_status {
                MergeStatus::Before => {
                    if new_interval[1] < interval[0] {
                        // This interval just fits here te current one starts are the one to insert starts
                        result.push(new_interval.clone());
                        result.push(interval);
                        merge_status = MergeStatus::After;
                    } else if new_interval[0] <= interval[1] {
                        merge_status = MergeStatus::During([
                            new_interval[0].min(interval[0]),
                            interval[1].max(new_interval[1]),
                        ]);
                    } else {
                        result.push(interval);
                    }
                }
                MergeStatus::During(current) => {
                    if interval[0] <= current[1] {
                        merge_status =
                            MergeStatus::During([current[0], current[1].max(interval[1])]);
                    } else {
                        result.push(current.to_vec());
                        result.push(interval);
                        merge_status = MergeStatus::After;
                    }
                }
                MergeStatus::After => result.push(interval),
            }
        }
        match merge_status {
            MergeStatus::Before => result.push(new_interval),
            MergeStatus::During(merged_interval) => result.push(merged_interval.to_vec()),
            MergeStatus::After => {}
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
