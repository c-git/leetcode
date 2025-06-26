//! Solution for https://leetcode.com/problems/count-ways-to-group-overlapping-ranges
//! 2580. Count Ways to Group Overlapping Ranges

impl Solution {
    pub fn count_ways(mut ranges: Vec<Vec<i32>>) -> i32 {
        ranges.sort_unstable();
        let mut result = 2;
        let mut last_end_time = ranges[0][1];
        for range in ranges.into_iter().skip(1) {
            let start = range[0];
            let end = range[1];
            if last_end_time < start {
                result = (result * 2) % 1_000_000_007;
            }
            last_end_time = last_end_time.max(end);
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
    #[case(vec![vec![6,10],vec![5,15]], 2)]
    #[case(vec![vec![1,3],vec![10,20],vec![2,5],vec![4,8]], 4)]
    fn case(#[case] ranges: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::count_ways(ranges);
        assert_eq!(actual, expected);
    }
}
