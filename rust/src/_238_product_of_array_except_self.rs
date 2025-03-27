//! Solution for https://leetcode.com/problems/product-of-array-except-self
//! 238. Product of Array Except Self

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(nums.len());

        // Fill result with prefix products
        for num in nums.iter().copied() {
            let next_prod = num * result.last().copied().unwrap_or(1);
            result.push(next_prod);
        }

        // Calculate a running suffix and use that to set correct final values
        let mut suffix = nums.last().copied().expect("guaranteed at least 2 values");
        *result.last_mut().unwrap() = result[result.len() - 2];
        for i in (1..nums.len() - 1).rev() {
            result[i] = result[i - 1] * suffix;
            suffix *= nums[i];
        }
        result[0] = suffix;

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
