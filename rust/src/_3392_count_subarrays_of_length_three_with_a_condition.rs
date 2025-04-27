//! Solution for https://leetcode.com/problems/count-subarrays-of-length-three-with-a-condition
//! 3392. Count Subarrays of Length Three With a Condition

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for window in nums.windows(3){
            if window[1] % 2 == 0 && (window[0] + window[2]) == window[1]/2 {
                result += 1;
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
    #[case(vec![1,2,1,4,1], 1)]
    #[case(vec![1,1,1], 0)]
    #[case(vec![0,-4,-4], 0)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::count_subarrays(nums);
        assert_eq!(actual, expected);
    }
}
