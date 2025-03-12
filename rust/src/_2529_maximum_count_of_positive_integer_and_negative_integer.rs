//! Solution for https://leetcode.com/problems/maximum-count-of-positive-integer-and-negative-integer
//! 2529. Maximum Count of Positive Integer and Negative Integer

impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        let separator_index = match nums.binary_search(&0) {
            Ok(x) => x,
            Err(x) => x,
        };

        let mut max_negative_index = separator_index;
        while max_negative_index > 0 && nums[max_negative_index] >= 0 {
            max_negative_index -= 1;
        }

        let mut min_positive_index = separator_index;
        while min_positive_index < nums.len() && nums[min_positive_index] <= 0 {
            min_positive_index += 1;
        }

        (max_negative_index + if nums[max_negative_index] < 0 { 1 } else { 0 })
            .max(nums.len() - min_positive_index) as _
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![-2,-1,-1,1,2,3], 3)]
    #[case(vec![-3,-2,-1,0,0,1,2], 3)]
    #[case(vec![5,20,66,1314], 4)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::maximum_count(nums);
        assert_eq!(actual, expected);
    }
}
