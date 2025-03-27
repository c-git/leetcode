//! Solution for https://leetcode.com/problems/product-of-array-except-self
//! 238. Product of Array Except Self

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut prefix = Vec::with_capacity(nums.len());
        let mut suffix = vec![0; nums.len()];
        for num in nums.iter().copied() {
            let next_prod = num * prefix.last().copied().unwrap_or(1);
            prefix.push(next_prod);
        }

        *suffix.last_mut().unwrap() = nums.last().cloned().unwrap();
        for i in (0..nums.len() - 1).rev() {
            suffix[i] = nums[i] * suffix[i + 1];
        }

        let mut result = Vec::with_capacity(nums.len());
        result.push(suffix[1]); // Product of all but first value
        for i in 1..nums.len() - 1 {
            result.push(prefix[i - 1] * suffix[i + 1]);
        }
        result.push(prefix[nums.len() - 2]);

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
    #[case(vec![1,2,3,4], vec![24,12,8,6])]
    #[case(vec![-1,1,0,-3,3], vec![0,0,9,0,0])]
    fn case(#[case] nums: Vec<i32>, #[case] expected: Vec<i32>) {
        let actual = Solution::product_except_self(nums);
        assert_eq!(actual, expected);
    }
}
