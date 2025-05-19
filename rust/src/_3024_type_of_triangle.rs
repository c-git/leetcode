//! Solution for https://leetcode.com/problems/type-of-triangle
//! 3024. Type of Triangle

impl Solution {
    pub fn triangle_type(mut nums: Vec<i32>) -> String {
        nums.sort_unstable();
        match (nums[0], nums[1], nums[2]) {
            (a, b, c) if a + b < c => "none",
            (a, b, c) if a == b && b == c => "equilateral",
            (a, b, c) if a == b || b == c => "isosceles",
            _ => "scalene",
        }
        .to_string()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![3,3,3], "equilateral")]
    #[case(vec![3,4,5], "scalene")]
    fn case(#[case] nums: Vec<i32>, #[case] expected: String) {
        let actual = Solution::triangle_type(nums);
        assert_eq!(actual, expected);
    }
}
