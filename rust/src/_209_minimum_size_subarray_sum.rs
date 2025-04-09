//! Solution for https://leetcode.com/problems/minimum-size-subarray-sum
//! 209. Minimum Size Subarray Sum

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut result = i32::MAX;
        let mut left = 0;
        let mut sum = 0;
        for (right, num) in nums.iter().enumerate() {
            sum += num;                            
            while sum >= target {
                result = result.min((right-left+1) as i32);
                sum -= nums[left];
                left += 1;
            }
        }
        if result == i32::MAX {
            0
        } else {
            result
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
    #[case(7, vec![2,3,1,2,4,3], 2)]
    #[case(4, vec![1,4,4], 1)]
    #[case(11, vec![1,1,1,1,1,1,1,1], 0)]
    fn case(#[case] target: i32, #[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::min_sub_array_len(target, nums);
        assert_eq!(actual, expected);
    }
}
