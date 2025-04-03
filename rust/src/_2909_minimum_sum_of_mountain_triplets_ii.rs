//! Solution for https://leetcode.com/problems/minimum-sum-of-mountain-triplets-ii
//! 2909. Minimum Sum of Mountain Triplets II

impl Solution {
    pub fn minimum_sum(nums: Vec<i32>) -> i32 {
        let mut result: Option<i32> = None;
        let mut min = nums.first().copied().unwrap();
        let min_prefix: Vec<_> = nums
            .iter()
            .copied()
            .map(|x| {
                min = min.min(x);
                min
            })
            .collect();
        let mut k_val = nums.last().copied().unwrap();
        for (j, j_val) in nums
            .iter()
            .copied()
            .enumerate()
            .take(nums.len() - 1)
            .skip(1)
            .rev()
        {
            if min_prefix[j - 1] < j_val && k_val < j_val {
                let sum = min_prefix[j - 1] + j_val + k_val;
                if let Some(res) = result {
                    result = Some(res.min(sum));
                } else {
                    result = Some(sum);
                }
            }
            k_val = k_val.min(j_val);
        }
        result.unwrap_or(-1)
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![8,6,1,5,3], 9)]
    #[case(vec![5,4,8,7,10,2], 13)]
    #[case(vec![6,5,4,3,4,5], -1)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::minimum_sum(nums);
        assert_eq!(actual, expected);
    }
}
