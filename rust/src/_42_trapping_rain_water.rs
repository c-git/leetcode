//! Solution for https://leetcode.com/problems/trapping-rain-water
//! 42. Trapping Rain Water

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut left_peak = height[left];
        let mut right_peak = height[right];
        while left < right {
            let min_peak = left_peak.min(right_peak); // Height we can collect water at

            // Move in lower side trying to find a new peak
            if height[left] < height[right] {
                if min_peak > height[left] {
                    result += min_peak - height[left];
                }
                left += 1;
                left_peak = left_peak.max(height[left]);
            } else {
                if min_peak > height[right] {
                    result += min_peak - height[right];
                }
                right -= 1;
                right_peak = right_peak.max(height[right]);
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
    fn case(#[case] height: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::trap(height);
        assert_eq!(actual, expected);
    }
}
