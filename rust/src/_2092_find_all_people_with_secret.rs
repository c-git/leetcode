//! Solution for https://leetcode.com/problems/find-all-people-with-secret
//! 2092. Find All People With Secret

use std::collections::BTreeSet;

impl Solution {
    pub fn find_all_people(_n: i32, mut meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
        meetings.sort_unstable_by(|x, y| x[2].cmp(&y[2]));
        let mut result: BTreeSet<i32> = [0, first_person].into();
        let mut iter = meetings.into_iter().peekable();
        while let Some(meeting) = iter.next() {
            let mut curr_meetings = vec![meeting];

            // Collect all the current meetings
            while Some(true) == iter.peek().map(|x| x[2] == curr_meetings[0][2]) {
                curr_meetings.push(iter.next().expect("just checked that it is some"));
            }

            // See if anyone in the current meetings knows the secret
            if curr_meetings
                .iter()
                .any(|x| result.contains(&x[0]) || result.contains(&x[1]))
            {
                // Someone knows the secret spread too all people currently in meetings
                curr_meetings.iter().for_each(|x| {
                    result.insert(x[0]);
                    result.insert(x[1]);
                });
            }
        }
        result.into_iter().collect()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(6, vec![vec![1,2,5],vec![2,3,8],vec![1,5,10]], 1, vec![0,1,2,3,5])]
    #[case(4, vec![vec![3,1,3],vec![1,2,2],vec![0,3,3]], 3, vec![0,1,3])]
    #[case(5, vec![vec![3,4,2],vec![1,2,1],vec![2,3,1]], 1, vec![0,1,2,3,4])]
    fn case(
        #[case] n: i32,
        #[case] meetings: Vec<Vec<i32>>,
        #[case] first_person: i32,
        #[case] mut expected: Vec<i32>,
    ) {
        let mut actual = Solution::find_all_people(n, meetings, first_person);
        actual.sort();
        expected.sort();
        assert_eq!(actual, expected);
    }
}
