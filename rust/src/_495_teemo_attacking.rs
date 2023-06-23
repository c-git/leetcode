//! Solution for https://leetcode.com/problems/teemo-attacking
//! 495. Teemo Attacking

impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let mut result = 0;
        let mut attack_start = 0;
        let mut attack_end = 0;
        for time in time_series {
            if time > attack_end {
                result += attack_end - attack_start;
                attack_start = time;
            }
            attack_end = time + duration;
        }
        result += attack_end - attack_start;
        result
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,4], 2, 4)]
    #[case(vec![1,2], 2, 3)]
    fn case(#[case] time_series: Vec<i32>, #[case] duration: i32, #[case] expected: i32) {
        let actual = Solution::find_poisoned_duration(time_series, duration);
        assert_eq!(actual, expected);
    }
}
