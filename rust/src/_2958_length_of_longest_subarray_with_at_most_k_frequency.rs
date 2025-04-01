//! Solution for https://leetcode.com/problems/length-of-longest-subarray-with-at-most-k-frequency
//! 2958. Length of Longest Subarray With at Most K Frequency

impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut left = 0;
        let mut result = 1;
        let mut freq_count = std::collections::BTreeMap::new();
        freq_count.insert(nums[left], 1);

        for (right, new_value) in nums.iter().copied().enumerate().skip(1) {
            let freq = freq_count.entry(new_value).or_default();
            *freq += 1;
            if *freq > k {
                // exceeded limit, move forward until we find a matching value to restore constraint
                result = result.max(right - left); // Plus 1 not needed because we are not including the right end
                loop {
                    let key = nums[left];
                    let freq = freq_count
                        .get_mut(&key)
                        .expect("must exist because we added it when we passed");
                    *freq -= 1;
                    if *freq == 0 {
                        freq_count.remove(&key);
                    }
                    left += 1;
                    if key == new_value {
                        break;
                    }
                }
            }
        }
        result = result.max(nums.len() - left);
        result as _
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,3,1,2,3,1,2], 2, 6)]
    #[case(vec![1,2,1,2,1,2,1,2], 1, 2)]
    #[case(vec![5,5,5,5,5,5,5], 4, 4)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::max_subarray_length(nums, k);
        assert_eq!(actual, expected);
    }
}
