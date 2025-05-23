//! Solution for https://leetcode.com/problems/find-the-maximum-sum-of-node-values
//! 3068. Find the Maximum Sum of Node Values

impl Solution {
    /// After skimming the editorial but only really taking away the concept of
    /// of chains allowing you to have an effective operation on any pair of
    /// nodes
    pub fn maximum_value_sum(nums: Vec<i32>, k: i32, _edges: Vec<Vec<i32>>) -> i64 {
        let mut result = 0;
        let mut max_unchanged = i32::MIN;
        let mut min_changed = i32::MAX;
        let mut is_even_changed = true;
        for num in nums {
            if num ^ k > num {
                result += (num ^ k) as i64;
                min_changed = min_changed.min(num);
                is_even_changed = !is_even_changed;
            } else {
                result += num as i64;
                max_unchanged = max_unchanged.max(num);
            }
        }
        if !is_even_changed {
            let delta_changed = (min_changed - (min_changed ^ k)).abs();
            let delta_unchanged = (max_unchanged - (max_unchanged ^ k)).abs();
            if delta_changed < delta_unchanged {
                result -= delta_changed as i64;
            } else {
                result -= delta_unchanged as i64;
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
    #[case(vec![1,2,1], 3, vec![vec![0,1],vec![0,2]], 6)]
    #[case(vec![2,3], 7, vec![vec![0,1]], 9)]
    #[case(vec![7,7,7,7,7,7], 3, vec![vec![0,1],vec![0,2],vec![0,3],vec![0,4],vec![0,5]], 42)]
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
