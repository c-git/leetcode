//! Solution for https://leetcode.com/problems/maximum-product-of-first-and-last-elements-of-a-subsequence
//! 3584. Maximum Product of First and Last Elements of a Subsequence

impl Solution {
    pub fn maximum_product(nums: Vec<i32>, m: i32) -> i64 {
        let mut result = i64::MIN;
        let n = nums.len();
        let m = m as usize;
        let mut temp = i64::MAX;
        let min_prefix: Vec<i64> = nums
            .iter()
            .map(|x| {
                temp = temp.min(*x as i64);
                temp
            })
            .collect();
        temp = i64::MIN;
        let max_prefix: Vec<i64> = nums
            .iter()
            .map(|x| {
                temp = temp.max(*x as i64);
                temp
            })
            .collect();
        let mut temp = i64::MAX;
        let min_suffix: Vec<i64> = nums
            .iter()
            .rev()
            .map(|x| {
                temp = temp.min(*x as i64);
                temp
            })
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();
        let mut temp = i64::MIN;
        let max_suffix: Vec<i64> = nums
            .iter()
            .rev()
            .map(|x| {
                temp = temp.max(*x as i64);
                temp
            })
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();
        for first in 0..n - m + 1 {
            let last = first + m - 1;
            result = result.max(min_prefix[first] * min_suffix[last]);
            result = result.max(min_prefix[first] * max_suffix[last]);
            result = result.max(max_prefix[first] * min_suffix[last]);
            result = result.max(max_prefix[first] * max_suffix[last]);
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
    #[case(vec![-1,-9,2,3,-2,-3,1], 1, 81)]
    #[case(vec![1,3,-5,5,6,-4], 3, 20)]
    #[case(vec![2,-1,2,-6,5,2,-5,7], 2, 35)]
    fn case(#[case] nums: Vec<i32>, #[case] m: i32, #[case] expected: i64) {
        let actual = Solution::maximum_product(nums, m);
        assert_eq!(actual, expected);
    }
}
