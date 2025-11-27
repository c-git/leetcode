//! Solution for https://leetcode.com/problems/maximum-subarray-sum-with-length-divisible-by-k
//! 3381. Maximum Subarray Sum With Length Divisible by K

impl Solution {
    pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut result = i64::MIN;

        // Brute force, try all sizes that are multiples of k
        for multiple in 1.. {
            let size = k * multiple;
            if size > nums.len() {
                // No longer fits
                break;
            }
            result = result.max(Self::max_subarray_sum_of_size(&nums, size));
        }
        result
    }

    fn max_subarray_sum_of_size(nums: &[i32], size: usize) -> i64 {
        let mut current: i64 = nums.iter().map(|&x| x as i64).take(size).sum();
        let mut result = current;
        for right in size..nums.len() {
            let left = right - size;
            current -= nums[left] as i64;
            current += nums[right] as i64;
            result = result.max(current);
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
    #[case(vec![1,2], 1, 3)]
    #[case(vec![-1,-2,-3,-4,-5], 4, -10)]
    #[case(vec![-5,1,2,-3,4], 2, 4)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i64) {
        let actual = Solution::max_subarray_sum(nums, k);
        assert_eq!(actual, expected);
    }
}
