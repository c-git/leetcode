//! Solution for https://leetcode.com/problems/keep-multiplying-found-values-by-two
//! 2154. Keep Multiplying Found Values by Two

impl Solution {
    pub fn find_final_value(mut nums: Vec<i32>, mut original: i32) -> i32 {
        nums.sort_unstable();
        while nums.binary_search(&original).is_ok() {
            original *= 2;
        }
        original
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![5,3,6,1,12], 3, 24)]
    #[case(vec![2,7,9], 4, 4)]
    fn case(#[case] nums: Vec<i32>, #[case] original: i32, #[case] expected: i32) {
        let actual = Solution::find_final_value(nums, original);
        assert_eq!(actual, expected);
    }
}
