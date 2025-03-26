//! Solution for https://leetcode.com/problems/majority-element
//! 169. Majority Element

use std::collections::HashMap;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut hash_map: HashMap<i32, usize> = HashMap::new();
        let threshold = nums.len() / 2;
        for num in nums {
            *hash_map.entry(num).or_default() += 1;
        }
        hash_map
            .into_iter()
            .find_map(|(num, freq)| if freq > threshold { Some(num) } else { None })
            .expect("You may assume that the majority element always exists in the array")
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![3,2,3], 3)]
    #[case(vec![2,2,1,1,1,2,2], 2)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::majority_element(nums);
        assert_eq!(actual, expected);
    }
}
