//! Solution for https://leetcode.com/problems/count-number-of-bad-pairs
//! 2364. Count Number of Bad Pairs

impl Solution {
    /// After using hints    
    pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        let total_pairs = (1..nums.len()).fold(0, |acc, x| acc + x as i64);
        let mut good_pairs = 0;
        let mut seen: std::collections::HashMap<i32, u32> = Default::default();
        for (i, num) in nums.into_iter().enumerate() {
            let diff = num - i as i32;
            let entry = seen.entry(diff).or_default();
            good_pairs += *entry;
            *entry += 1;
        }

        total_pairs - good_pairs as i64
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![4,1,3,3], 5)]
    #[case(vec![1,2,3,4,5], 0)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i64) {
        let actual = Solution::count_bad_pairs(nums);
        assert_eq!(actual, expected);
    }
}
