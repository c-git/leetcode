//! Solution for https://leetcode.com/problems/daily-temperatures
//! 739. Daily Temperatures

use std::collections::BTreeMap;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<Option<usize>> = vec![None; temperatures.len()];
        let mut last_seen: BTreeMap<i32, usize> = BTreeMap::new();
        for i in (0..temperatures.len()).rev() {
            let today_temperature = temperatures[i];
            for (_key, prev_index) in last_seen.range((today_temperature + 1)..) {
                let this_diff = prev_index - i;
                result[i] = match result[i] {
                    Some(curr) => Some(curr.min(this_diff)),
                    None => Some(this_diff),
                };
            }
            last_seen.insert(today_temperature, i);
        }
        result
            .into_iter()
            .map(|x| x.unwrap_or_default() as i32)
            .collect()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![73,74,75,71,69,72,76,73], vec![1,1,4,2,1,1,0,0])]
    #[case(vec![30,40,50,60], vec![1,1,1,0])]
    #[case(vec![30,60,90], vec![1,1,0])]
    fn case(#[case] temperatures: Vec<i32>, #[case] expected: Vec<i32>) {
        let actual = Solution::daily_temperatures(temperatures);
        assert_eq!(actual, expected);
    }
}
