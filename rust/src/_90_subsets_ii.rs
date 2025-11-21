//! Solution for https://leetcode.com/problems/subsets-ii
//! 90. Subsets II

impl Solution {
    /// Based on https://youtube.com/watch?v=Vn2v6ajA7U0&si=kbCMeZbfuXCxG_mX
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.is_empty() {
            return vec![];
        }

        let mut result = vec![];
        nums.sort_unstable();
        let mut partial = vec![];
        Self::subsets_with_dup_(&nums, &mut partial, &mut result);
        result
    }

    fn subsets_with_dup_(nums: &[i32], partial: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if let Some(&first) = nums.first() {
            // Including first number
            partial.push(first);
            Self::subsets_with_dup_(&nums[1..], partial, result);
            partial.pop();

            // With any more of the first number (skip all duplicates)
            let mut i = 1;
            while i < nums.len() && nums[i] == nums[i - 1] {
                i += 1;
            }
            Self::subsets_with_dup_(&nums[i..], partial, result);
        } else {
            result.push(partial.clone());
        }
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,2], vec![vec![],vec![1],vec![1,2],vec![1,2,2],vec![2],vec![2,2]])]
    #[case(vec![0], vec![vec![],vec![0]])]
    fn case(#[case] nums: Vec<i32>, #[case] mut expected: Vec<Vec<i32>>) {
        let mut actual = Solution::subsets_with_dup(nums);
        actual.iter_mut().for_each(|x| x.sort_unstable());
        expected.iter_mut().for_each(|x| x.sort_unstable());
        actual.sort_unstable();
        expected.sort_unstable();
        assert_eq!(actual, expected);
    }
}
