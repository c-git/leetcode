//! Solution for https://leetcode.com/problems/product-of-array-except-self
//! 238. Product of Array Except Self

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; nums.len()];
        let mut partial_product = 1;
        let prefix_prods: Vec<i32> = nums
            .iter()
            .map(|x| {
                partial_product *= x;
                partial_product
            })
            .collect();

        // Set end value
        result[nums.len() - 1] = prefix_prods[prefix_prods.len() - 2];

        // Set middle values (skip both ends)
        partial_product = nums[nums.len() - 1];

        for (i, result_num) in result.iter_mut().enumerate().skip(1).rev().skip(1) {
            *result_num = prefix_prods[i - 1] * partial_product;
            partial_product *= nums[i];
        }
        result[0] = partial_product; // Store product of everything but first

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
