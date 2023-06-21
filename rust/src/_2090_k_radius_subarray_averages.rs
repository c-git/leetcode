//! Solution for https://leetcode.com/problems/k-radius-subarray-averages
//! 2090. K Radius Subarray Averages

impl Solution {
    pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        if k == 0 {
            return nums;
        }
        if k > (nums.len() / 2 + 1) {
            return vec![-1; nums.len()];
        }
        let mut result = vec![-1; nums.len()];
        let mut sum: i64 = nums.iter().take(k * 2).map(|x| *x as i64).sum();
        let denominator = k as i64 * 2 + 1;
        for i in k..(nums.len() - k) {
            sum += nums[i + k] as i64;
            result[i] = (sum / denominator) as i32;
            sum -= nums[i - k] as i64;
        }
        result
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![7,4,3,9,1,8,5,2,6], 3, vec![-1,-1,-1,5,4,4,-1,-1,-1])]
    #[case(vec![100000], 0, vec![100000])]
    #[case(vec![8], 100000, vec![-1])]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: Vec<i32>) {
        let actual = Solution::get_averages(nums, k);
        assert_eq!(actual, expected);
    }
}
