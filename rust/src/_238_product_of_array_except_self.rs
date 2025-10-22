//! Solution for https://leetcode.com/problems/product-of-array-except-self
//! 238. Product of Array Except Self

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut partial_product = 1;
        let mut result: Vec<i32> = nums
            .iter()
            .map(|num| {
                partial_product *= num;
                partial_product
            })
            .collect();
        partial_product = 1;
        for (i, num) in nums.iter().enumerate().skip(1).rev() {
            result[i] = partial_product * result[i - 1];
            partial_product *= num;
        }
        result[0] = partial_product;
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
