//! Solution for https://leetcode.com/problems/daily-temperatures
//! 739. Daily Temperatures

impl Solution {
    pub fn daily_temperatures(mut temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<(i32, usize)> = vec![];
        for (i, temperature) in temperatures.iter_mut().enumerate().rev() {
            // Remove all smaller or equal temps from the stack
            while let Some((later_temp, _later_idx)) = stack.last() {
                if later_temp <= temperature {
                    stack.pop();
                } else {
                    break;
                }
            }

            let days_until_warmer = stack.last().map(|(_, later_idx)| *later_idx).unwrap_or(i) - i;

            stack.push((*temperature, i));

            // Set the number of days for this value
            *temperature = days_until_warmer as _;
        }
        temperatures
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
