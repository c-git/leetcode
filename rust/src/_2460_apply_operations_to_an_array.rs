//! Solution for https://leetcode.com/problems/apply-operations-to-an-array
//! 2460. Apply Operations to an Array

impl Solution {
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 0..nums.len() - 1 {
            if nums[i] == nums[i + 1] {
                nums[i] *= 2;
                nums[i + 1] = 0;
            }
        }
        nums.sort_by_key(|&x| if x == 0 { 1 } else { 0 });
        nums
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,2,1,1,0], vec![1,4,2,0,0,0])]
    #[case(vec![0,1], vec![1,0])]
    fn case(#[case] nums: Vec<i32>, #[case] expected: Vec<i32>) {
        let actual = Solution::apply_operations(nums);
        assert_eq!(actual, expected);
    }
}
