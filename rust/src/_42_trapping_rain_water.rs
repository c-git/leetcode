//! Solution for https://leetcode.com/problems/trapping-rain-water
//! 42. Trapping Rain Water

impl Solution {
    pub fn trap(heights: Vec<i32>) -> i32 {
        // Based on https://www.youtube.com/watch?v=588iXKwb7Zs
        let mut result = 0;
        let mut left = 0;
        let mut right = heights.len() - 1;
        let mut max_left = heights[left];
        let mut max_right = heights[right];
        while left < right {
            if max_left < max_right {
                result += max_left - heights[left];
                left += 1;
                max_left = max_left.max(heights[left]);
            } else {
                result += max_right - heights[right];
                right -= 1;
                max_right = max_right.max(heights[right]);
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
    #[case(vec![0,1,0,2,1,0,1,3,2,1,2,1], 6)]
    #[case(vec![4,2,0,3,2,5], 9)]
    #[case(vec![8,0,1,1,5,3,4,0,1,2], 17)]
    #[case(vec![20,0,1,1,5,3,4,0,1,2], 17)]
    #[case(vec![1,2,3,4,5,20,0,1,1,5,3,4,0,1,2], 17)]
    #[case(vec![1,2,3,4,5], 0)]
    #[case(vec![5,4,3,2,1,0], 0)]
    #[case(vec![5,5,1,7,1,1,5,2,7,6], 23)]
    fn case(#[case] height: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::trap(height);
        assert_eq!(actual, expected);
    }
}
