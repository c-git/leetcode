//! Solution for https://leetcode.com/problems/search-in-rotated-sorted-array
//! 33. Search in Rotated Sorted Array

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let first = nums[0];
        let partition_point = nums.partition_point(|&x| x >= first);
        if partition_point >= nums.len() {
            // not rotated (or rotated by 0)
            return match nums.binary_search(&target) {
                Ok(idx) => idx as _,
                Err(_) => -1, // Not found
            };
        }
        if target >= first {
            match nums[..partition_point].binary_search(&target) {
                Ok(idx) => idx as _,
                Err(_) => -1, // Not in first section and cannot be in second
            }
        } else {
            match nums[partition_point..].binary_search(&target) {
                Ok(idx) => (idx + partition_point) as _,
                Err(_) => -1, // Not in first section and cannot be in second
            }
        }
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![4,5,6,7,0,1,2], 0, 4)]
    #[case(vec![4,5,6,7,0,1,2], 3, -1)]
    #[case(vec![1], 0, -1)]
    fn case(#[case] nums: Vec<i32>, #[case] target: i32, #[case] expected: i32) {
        let actual = Solution::search(nums, target);
        assert_eq!(actual, expected);
    }
}
