//! Solution for https://leetcode.com/problems/count-of-interesting-subarrays
//! 2845. Count of Interesting Subarrays

impl Solution {
    pub fn count_interesting_subarrays(mut nums: Vec<i32>, modulo: i32, k: i32) -> i64 {
        let mut result = 0i64;

        // Convert to prefix sum of number of interesting indices
        let mut prefix_sum = 0;
        for num in nums.iter_mut() {
            if *num % modulo == k {
                prefix_sum += 1;
            }
            *num = prefix_sum;
        }

        // Count number of matches with no subtraction
        result += nums.iter().filter(|&&cnt| cnt % modulo == k).count() as i64;

        // Count subtracting values at previous indices
        for (i, prev_count) in nums.iter().copied().enumerate() {
            result += nums
                .iter()
                .skip(i + 1)
                .filter(|&&cnt| (cnt - prev_count) % modulo == k)
                .count() as i64;
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
    #[case(vec![3,2,4], 2, 1, 3)]
    #[case(vec![3,1,9,6], 3, 0, 2)]
    fn case(#[case] nums: Vec<i32>, #[case] modulo: i32, #[case] k: i32, #[case] expected: i64) {
        let actual = Solution::count_interesting_subarrays(nums, modulo, k);
        assert_eq!(actual, expected);
    }
}
