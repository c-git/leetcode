//! Solution for https://leetcode.com/problems/subarray-sum-equals-k
//! 560. Subarray Sum Equals K

impl Solution {
    // Based on https://www.youtube.com/watch?v=fFVZt-6sgyo
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;
        let mut prev_sum_seen = std::collections::HashMap::with_capacity(nums.len());
        prev_sum_seen.insert(0, 1); // The empty prefix sums to 0
        let mut curr_sum = 0;
        for num in nums {
            curr_sum += num;
            if let Some(prev_sum_count) = prev_sum_seen.get(&(curr_sum - k)) {
                // Add the amount of ways we can make the difference we need to subtract to get `k`
                result += prev_sum_count;
            }

            // Add the current sum as one that can be subtracted
            *prev_sum_seen.entry(curr_sum).or_default() += 1;
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
    #[case(vec![1,1,1], 2, 2)]
    #[case(vec![1,2,3], 3, 2)]
    #[case(vec![1], 0, 0)]
    #[case(vec![-1,-1,1], 0, 1)]
    #[case(vec![28,54,7,-70,22,65,-6], 100, 1)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::subarray_sum(nums, k);
        assert_eq!(actual, expected);
    }
}
