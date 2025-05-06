//! Solution for https://leetcode.com/problems/build-array-from-permutation
//! 1920. Build Array from Permutation

impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        nums.iter()
            .map(|&num| num as usize)
            .map(|index| nums[index])
            .collect()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![0,2,1,5,3,4], vec![0,1,2,4,5,3])]
    #[case(vec![5,0,1,2,3,4], vec![4,5,0,1,2,3])]
    fn case(#[case] nums: Vec<i32>, #[case] expected: Vec<i32>) {
        let actual = Solution::build_array(nums);
        assert_eq!(actual, expected);
    }
}
