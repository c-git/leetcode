//! Solution for https://leetcode.com/problems/search-in-rotated-sorted-array
//! 33. Search in Rotated Sorted Array

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let partition_idx = nums.partition_point(|x| x >= &nums[0]);
        if target < nums[0] {
            nums[partition_idx..]
                .binary_search(&target)
                .map(|x| (x + partition_idx) as i32)
                .unwrap_or(-1)
        } else {
            nums[..partition_idx]
                .binary_search(&target)
                .map(|x| x as i32)
                .unwrap_or(-1)
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
    #[case(vec![0,1,2,3,4,5,6,7], 3, 3)]
    fn case(#[case] nums: Vec<i32>, #[case] target: i32, #[case] expected: i32) {
        let actual = Solution::search(nums, target);
        assert_eq!(actual, expected);
    }
}
