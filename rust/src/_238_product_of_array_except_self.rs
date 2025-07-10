//! Solution for https://leetcode.com/problems/product-of-array-except-self
//! 238. Product of Array Except Self

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        // Populate result with suffix products
        let mut result = vec![0; nums.len()];
        let mut suffix_product = 1;
        for (i, num) in nums.iter().enumerate().rev() {
            suffix_product *= num;
            result[i] = suffix_product;
        }
        result[0] = result[1];
        let mut prefix_product = nums[0];
        for (i, num) in nums.iter().enumerate().take(nums.len() - 1).skip(1) {
            result[i] = prefix_product * result[i + 1];
            prefix_product *= num;
        }
        let n = result.len();
        result[n - 1] = prefix_product;
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
