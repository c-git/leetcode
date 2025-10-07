//! Solution for https://leetcode.com/problems/daily-temperatures
//! 739. Daily Temperatures

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; temperatures.len()];
        // Stores a monotonically decreasing stack of temperatures and their indices
        let mut high_temps = vec![(
            *temperatures.last().expect("min expected length is 1"),
            temperatures.len() - 1,
        )];

        for (i, current_temperature) in temperatures
            .iter()
            .take(temperatures.len() - 1)
            .enumerate()
            .rev()
        {
            while let Some((top_value, top_idx)) = high_temps.last() {
                if top_value > current_temperature {
                    result[i] = (top_idx - i) as i32;
                    break;
                } else {
                    // We can no longer use this temp so remove it from the stack
                    high_temps.pop();
                }
            }
            // If result[i] was not updated then it should stay as 0 because there is nothing higher later on
            // Then always add on the current as it will always be lower than the top or the stack is empty
            high_temps.push((*current_temperature, i));
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
