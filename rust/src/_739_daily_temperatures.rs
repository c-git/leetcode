//! Solution for https://leetcode.com/problems/daily-temperatures
//! 739. Daily Temperatures

impl Solution {
    /// Noticed the pattern suggested while getting the problem to do on https://algomaster.io/practice/dsa-patterns
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut result = vec![-1; temperatures.len()];
        let mut highs_seen: Vec<(usize, i32)> = vec![];
        for (i, temperature) in temperatures.into_iter().enumerate().rev() {
            // Clear out smaller temperatures
            while let Some((_last_pos, last_temp)) = highs_seen.last() {
                if last_temp <= &temperature {
                    highs_seen.pop();
                } else {
                    break;
                }
            }
            // Get distance
            result[i] = if let Some((last_pos, last_temp)) = highs_seen.last() {
                debug_assert!(
                    *last_temp > temperature,
                    "We removed those equal or lower so this should always be true"
                );
                last_pos - i
            } else {
                0
            } as i32;

            // Save current if as smallest high seen
            highs_seen.push((i, temperature));
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
    #[case(vec![73,74,75,71,69,72,76,73], vec![1,1,4,2,1,1,0,0])]
    #[case(vec![30,40,50,60], vec![1,1,1,0])]
    #[case(vec![30,60,90], vec![1,1,0])]
    fn case(#[case] temperatures: Vec<i32>, #[case] expected: Vec<i32>) {
        let actual = Solution::daily_temperatures(temperatures);
        assert_eq!(actual, expected);
    }
}
