//! Solution for https://leetcode.com/problems/product-of-array-except-self
//! 238. Product of Array Except Self

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(nums.len());
        let mut prefix_prod = Vec::with_capacity(nums.len());
        let mut temp_prod = 1;
        for num in nums.iter() {
            temp_prod *= num;
            prefix_prod.push(temp_prod);
        }
        let mut suffix_prod = vec![1; nums.len()];
        temp_prod = 1;
        for (i, num) in nums.iter().enumerate().rev() {
            temp_prod *= num;
            suffix_prod[i] = temp_prod;
        }
        // For test case 1
        // nums:   [1,  2,   3,  4]
        // prefix: [1,  2,   6, 24]
        // suffix: [24, 24, 12,  4]
        // result: [24, 12,  8,  6]

        result.push(suffix_prod[1]);
        for i in 1..nums.len() - 1 {
            result.push(prefix_prod[i - 1] * suffix_prod[i + 1]);
        }
        result.push(prefix_prod[nums.len() - 2]);
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
