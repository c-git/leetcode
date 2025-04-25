//! Solution for https://leetcode.com/problems/subsets
//! 78. Subsets

impl Solution {
    /// Based on https://www.youtube.com/watch?v=REOH22Xwdkk
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        Self::dfs(&nums, 0, &mut vec![], &mut result);
        result
    }

    fn dfs(nums: &[i32], i: usize, subset: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if i >= nums.len() {
            result.push(subset.clone());
            return;
        }

        // Include nums[i]
        subset.push(nums[i]); // Add value to include it
        Self::dfs(nums, i + 1, subset, result);

        // Skip nums[i]
        subset.pop(); // Remove value added (Backtrack step)
        Self::dfs(nums, i + 1, subset, result);
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,3], vec![vec![],vec![1],vec![2],vec![1,2],vec![3],vec![1,3],vec![2,3],vec![1,2,3]])]
    #[case(vec![0], vec![vec![],vec![0]])]
    fn case(#[case] nums: Vec<i32>, #[case] mut expected: Vec<Vec<i32>>) {
        let mut actual = Solution::subsets(nums);
        actual.sort_unstable();
        expected.sort_unstable();
        assert_eq!(actual, expected);
    }
}
