//! Solution for https://leetcode.com/problems/count-largest-group
//! 1399. Count Largest Group

use std::collections::HashMap;

impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut freq: HashMap<i32, u32> = HashMap::new();
        for i in 1..=n {
            *freq.entry(Self::sum_digits(i)).or_default() += 1;
        }
        let mut max_value = 0;
        let mut max_freq = 0;
        for &value in freq.values() {
            match value.cmp(&max_value) {
                std::cmp::Ordering::Less => {} // Ignore value
                std::cmp::Ordering::Equal => max_freq += 1,
                std::cmp::Ordering::Greater => {
                    max_freq = 1;
                    max_value = value;
                }
            }
        }
        max_freq
    }

    fn sum_digits(mut x: i32) -> i32 {
        let mut result = 0;
        while x > 0 {
            result += x % 10;
            x /= 10;
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
    #[case(13, 4)]
    #[case(2, 2)]
    fn case(#[case] n: i32, #[case] expected: i32) {
        let actual = Solution::count_largest_group(n);
        assert_eq!(actual, expected);
    }
}
