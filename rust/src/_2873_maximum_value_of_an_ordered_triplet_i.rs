//! Solution for https://leetcode.com/problems/maximum-value-of-an-ordered-triplet-i
//! 2873. Maximum Value of an Ordered Triplet I

impl Solution {
    /// Based on https://www.youtube.com/watch?v=sFY_14A28x0
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let mut result = 0;
        let mut i_val = nums.first().copied().unwrap();
        for (j, j_val) in nums.iter().skip(1).copied().enumerate() {
            if i_val < j_val {
                i_val = j_val;
                continue;
            }
            for k_val in nums.iter().skip(j + 2).copied() {
                dbg!(
                    j,
                    i_val,
                    j_val,
                    k_val,
                    (i_val - j_val) as i64 * k_val as i64
                );
                result = result.max((i_val - j_val) as i64 * k_val as i64)
            }
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
    #[case(vec![12,6,1,2,7], 77)]
    #[case(vec![1,10,3,4,19], 133)]
    #[case(vec![1,2,3], 0)]
    #[case(vec![2,3,1], 0)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i64) {
        let actual = Solution::maximum_triplet_value(nums);
        assert_eq!(actual, expected);
    }
}
