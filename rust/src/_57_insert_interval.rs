//! Solution for https://leetcode.com/problems/insert-interval
//! 57. Insert Interval

enum MergeStatus {
    /// Before starting to merge
    Before,
    /// During the process of merging
    During([i32; 2]),
    /// New point either merged already inserted
    Completed,
}

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::with_capacity(intervals.len() + 1);
        let mut merge_status = MergeStatus::Before;
        for current_interval in intervals {
            match merge_status {
                MergeStatus::Before => {
                    if new_interval[1] < current_interval[0] {
                        // The new_interval just fits here before current_interval starts
                        result.push(new_interval.clone());
                        result.push(current_interval);
                        merge_status = MergeStatus::Completed;
                    } else if new_interval[0] <= current_interval[1] {
                        // The new_interval and the current_interval overlap, start the merge
                        merge_status = MergeStatus::During([
                            new_interval[0].min(current_interval[0]),
                            current_interval[1].max(new_interval[1]),
                        ]);
                    } else {
                        result.push(current_interval);
                    }
                }
                MergeStatus::During(merging) => {
                    if current_interval[0] <= merging[1] {
                        // Another overlap detected, continue merging
                        merge_status =
                            MergeStatus::During([merging[0], merging[1].max(current_interval[1])]);
                    } else {
                        // No more overlap insert the merged one and then the current_interval
                        result.push(merging.to_vec());
                        result.push(current_interval);
                        merge_status = MergeStatus::Completed;
                    }
                }
                MergeStatus::Completed => result.push(current_interval),
            }
        }
        match merge_status {
            MergeStatus::Before => result.push(new_interval),
            MergeStatus::During(merged_interval) => result.push(merged_interval.to_vec()),
            MergeStatus::Completed => {}
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
