//! Solution for https://leetcode.com/problems/alert-using-same-key-card-three-or-more-times-in-a-one-hour-period
//! 1604. Alert Using Same Key-Card Three or More Times in a One Hour Period

use std::collections::VecDeque;

impl Solution {
    pub fn alert_names(key_name: Vec<String>, key_time: Vec<String>) -> Vec<String> {
        let mut result = Vec::new();
        let mut key_name_pairs: Vec<(String, String)> =
            key_name.into_iter().zip(key_time).collect();
        key_name_pairs.sort_unstable();
        let mut current_name = key_name_pairs[0].0.clone();
        let mut recent_uses: VecDeque<(u8, u8)> = VecDeque::new();
        for (name, time) in key_name_pairs {
            // Check for if we are on a new employee
            if name == current_name {
                if Some(&name) == result.last() {
                    // This person already got an alert so we ignore the rest of their times
                    continue;
                }
            } else {
                // New employee update `current_name`
                recent_uses.clear();
                current_name = name.clone();
            }

            // Assumes all times are well formed
            let mut time_iter = time.split(":");
            let curr_hour = time_iter.next().unwrap().parse().unwrap();
            let curr_minute = time_iter.next().unwrap().parse().unwrap();
            recent_uses.push_back((curr_hour, curr_minute));
            while let Some(&(front_hour, front_minute)) = recent_uses.front() {
                if front_hour == curr_hour
                    || (front_hour + 1 == curr_hour && front_minute >= curr_minute)
                {
                    break;
                }
                recent_uses.pop_front();
            }
            if recent_uses.len() >= 3 {
                result.push(name);
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
    #[case(vec!["daniel".into(),"daniel".into(),"daniel".into(),"luis".into(),"luis".into(),"luis".into(),"luis".into()], vec!["10:00".into(),"10:40".into(),"11:00".into(),"09:00".into(),"11:00".into(),"13:00".into(),"15:00".into()], vec!["daniel".into()])]
    #[case(vec!["alice".into(),"alice".into(),"alice".into(),"bob".into(),"bob".into(),"bob".into(),"bob".into()], vec!["12:01".into(),"12:00".into(),"18:00".into(),"21:00".into(),"21:20".into(),"21:30".into(),"23:00".into()], vec!["bob".into()])]
    fn case(
        #[case] key_name: Vec<String>,
        #[case] key_time: Vec<String>,
        #[case] expected: Vec<String>,
    ) {
        let actual = Solution::alert_names(key_name, key_time);
        assert_eq!(actual, expected);
    }
}
