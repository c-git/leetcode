//! Solution for https://leetcode.com/problems/maximum-value-of-an-ordered-triplet-ii
//! 2874. Maximum Value of an Ordered Triplet II

impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let mut result = 0;
        let mut max = nums.first().copied().unwrap();
        let prefix_max: Vec<_> = nums
            .iter()
            .copied()
            .map(|x| {
                max = max.max(x);
                max
            })
            .collect();
        max = nums.last().copied().unwrap();
        let suffix_max: Vec<_> = nums
            .iter()
            .rev()
            .copied()
            .map(|x| {
                max = max.max(x);
                max
            })
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();
        let n = nums.len();
        for (j, j_val) in nums.into_iter().enumerate().take(n - 1).skip(1) {
            result = result.max((prefix_max[j - 1] - j_val) as i64 * suffix_max[j + 1] as i64);
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
    fn case(#[case] nums: Vec<i32>, #[case] expected: i64) {
        let actual = Solution::maximum_triplet_value(nums);
        assert_eq!(actual, expected);
    }
}
