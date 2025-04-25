//! Solution for https://leetcode.com/problems/permutations
//! 46. Permutations

impl Solution {
    /// Based on https://www.youtube.com/watch?v=s7AvT7cGdSo
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        Self::permute_(&mut nums)
    }

    pub fn permute_(nums: &mut Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];

        // Base case
        if nums.len() == 1 {
            return vec![nums.clone()];
        }

        for _ in 0..nums.len() {
            let first = nums.remove(0);
            let sub_results = Self::permute_(nums);

            for mut sub_result in sub_results {
                sub_result.push(first);
                result.push(sub_result);
            }
            nums.push(first);
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
    #[case(vec![1,2,3], vec![vec![1,2,3],vec![1,3,2],vec![2,1,3],vec![2,3,1],vec![3,1,2],vec![3,2,1]])]
    #[case(vec![0,1], vec![vec![0,1],vec![1,0]])]
    #[case(vec![1], vec![vec![1]])]
    fn case(#[case] nums: Vec<i32>, #[case] mut expected: Vec<Vec<i32>>) {
        let mut actual = Solution::permute(nums);
        actual.sort_unstable();
        expected.sort_unstable();
        assert_eq!(actual, expected);
    }
}
