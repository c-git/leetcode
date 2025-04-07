//! Solution for https://leetcode.com/problems/container-with-most-water
//! 11. Container With Most Water

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut left = 0;
        let mut right = height.len() - 1;
        while left < right {
            result = result.max(height[left].min(height[right]) * (right - left) as i32);
            if left < right {
                left += 1;
            } else {
                right -= 1;
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
    #[case(vec![1,8,6,2,5,4,8,3,7], 49)]
    #[case(vec![1,1], 1)]
    fn case(#[case] height: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::max_area(height);
        assert_eq!(actual, expected);
    }
}
