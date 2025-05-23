//! Solution for https://leetcode.com/problems/find-the-maximum-sum-of-node-values
//! 3068. Find the Maximum Sum of Node Values

impl Solution {
    /// After skimming the editorial but only really taking away the concept of
    /// of chains allowing you to have an effective operation on any pair of
    /// nodes
    pub fn maximum_value_sum(nums: Vec<i32>, k: i32, _edges: Vec<Vec<i32>>) -> i64 {
        let mut result = 0;
        let mut min_delta = (nums[0] ^ k) - nums[0];
        let mut is_even_changed = true;
        for num in nums {
            result += num as i64;
            let delta = (num ^ k) - num;
            if delta > 0 {
                result += delta as i64;
                is_even_changed = !is_even_changed;
            }
            if delta.abs() < min_delta.abs() {
                min_delta = delta;
            }
        }
        if !is_even_changed {
            result += min_delta as i64;
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
    #[case(vec![1,2,1], 3, vec![vec![0,1],vec![0,2]], 6)]
    #[case(vec![2,3], 7, vec![vec![0,1]], 9)]
    #[case(vec![7,7,7,7,7,7], 3, vec![vec![0,1],vec![0,2],vec![0,3],vec![0,4],vec![0,5]], 42)]
    #[case(vec![24,78,1,97,44], 6, vec![], 260)] //edges not copied
    fn case(
        #[case] nums: Vec<i32>,
        #[case] k: i32,
        #[case] edges: Vec<Vec<i32>>,
        #[case] expected: i64,
    ) {
        let actual = Solution::maximum_value_sum(nums, k, edges);
        assert_eq!(actual, expected);
    }
}
