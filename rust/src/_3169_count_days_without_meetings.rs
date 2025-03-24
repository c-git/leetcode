//! Solution for https://leetcode.com/problems/count-days-without-meetings
//! 3169. Count Days Without Meetings

impl Solution {
    pub fn count_days(days: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let mut meeting_end_day = 0;
        meetings.sort();
        for meeting in meetings {
            let (start_day, end_day) = (meeting[0], meeting[1]);
            if start_day > days {
                // This meeting starts after they are finished working
                break;
            }

            // Capture days in between if any
            if meeting_end_day < start_day {
                result += start_day - meeting_end_day - 1;
            }

            meeting_end_day = meeting_end_day.max(end_day);
        }

        if meeting_end_day < days {
            result += days - meeting_end_day;
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
    #[case(10, vec![vec![5,7],vec![1,3],vec![9,10]], 2)]
    #[case(5, vec![vec![2,4],vec![1,3]], 1)]
    #[case(6, vec![vec![1,6]], 0)]
    fn case(#[case] days: i32, #[case] meetings: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::count_days(days, meetings);
        assert_eq!(actual, expected);
    }
}
