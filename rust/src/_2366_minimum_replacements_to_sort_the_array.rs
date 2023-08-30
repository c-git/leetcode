//! Solution for https://leetcode.com/problems/minimum-replacements-to-sort-the-array
//! 2366. Minimum Replacements to Sort the Array

impl Solution {
    pub fn minimum_replacement(nums: Vec<i32>) -> i64 {
        let mut result = 0;
        let mut max = *nums.last().unwrap();
        for &num in nums.iter().rev() {
            if num > max {
                let min_splits = num / max;
                result += min_splits as i64;
                if num % max != 0 {
                    max = (num % max + max) / 2;
                } else {
                    result -= 1;
                }
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
    #[case(vec![3,9,3], 2)]
    #[case(vec![1,2,3,4,5], 0)]
    #[case(vec![50,9,3], 18)]
    #[case(vec![50,3,3], 16)]
    #[case(vec![50,1], 49)]
    #[case(vec![50,24], 2)]
    #[case(vec![50,16], 3)]
    #[case(vec![50,17], 2)]
    #[case(vec![16, 50, 16], 4)]
    #[case(vec![10, 50, 16], 3)]
    #[case(vec![12, 50, 16], 3)]
    #[case(vec![13, 50, 16], 4)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i64) {
        let actual = Solution::minimum_replacement(nums);
        assert_eq!(actual, expected);
    }
}
