//! Solution for https://leetcode.com/problems/rabbits-in-forest
//! 781. Rabbits in Forest

use std::collections::HashMap;

impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut seen: HashMap<i32, u16> = HashMap::new();
        for answer in answers {
            let entry = seen.entry(answer).or_default();
            if *entry == 0 {
                result += answer + 1;
            }
            *entry += 1;
            if *entry as i32 == answer + 1 {
                *entry = 0;
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
    #[case(vec![1,1,2], 5)]
    #[case(vec![10,10,10], 11)]
    fn case(#[case] answers: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::num_rabbits(answers);
        assert_eq!(actual, expected);
    }
}
