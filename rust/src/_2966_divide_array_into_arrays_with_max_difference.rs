//! Solution for https://leetcode.com/problems/divide-array-into-arrays-with-max-difference
//! 2966. Divide Array Into Arrays With Max Difference

impl Solution {
    pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        debug_assert_eq!(nums.len() % 3, 0, "By problem constraints");
        let mut result_arrays = vec![Vec::with_capacity(3); nums.len() / 3];
        let mut nums_iter = nums.into_iter();
        for result_array in result_arrays.iter_mut() {
            for _ in 0..3 {
                result_array.push(nums_iter.next().unwrap());
            }
            if result_array[2] - result_array[0] > k {
                return vec![];
            }
        }
        debug_assert!(nums_iter.next().is_none(), "Should be empty after loop");
        result_arrays
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,3,4,8,7,9,3,5,1], 2, vec![vec![1,1,3],vec![3,4,5],vec![7,8,9]])]
    #[case(vec![1,3,3,2,7,3], 3, vec![])]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: Vec<Vec<i32>>) {
        let actual = Solution::divide_array(nums, k);
        assert_eq!(actual, expected);
    }
}
