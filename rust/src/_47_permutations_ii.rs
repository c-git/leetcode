//! Solution for https://leetcode.com/problems/permutations-ii
//! 47. Permutations II

impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        // Copied from problem 31 and modified to suit
        let mut result = vec![];
        nums.sort_unstable();
        result.push(nums.clone());
        while !Self::next_permutation(&mut nums) {
            result.push(nums.clone());
        }
        result
    }

    /// Returns true if all values had to be reversed (indicated we are back to the start)
    fn next_permutation(nums: &mut Vec<i32>) -> bool {
        let n = nums.len();
        if n == 1 {
            return true;
        }

        // Find first pair of values where nums[i] < nums[i+1]
        let mut i = n - 2;
        loop {
            if nums[i] < nums[i + 1] {
                break;
            }
            if i > 0 {
                i -= 1;
            } else {
                // Special case when all values need to be reversed
                nums.reverse();
                return true;
            }
        }

        // Switch nums[i] with next element larger than it
        // Walk through values to the right of nums[i] and find the min that is still larger
        let right_value_index = nums
            .iter()
            .enumerate()
            .rev()
            .find(|(_index, &x)| x > nums[i])
            .map(|(index, _x)| index)
            .unwrap();
        nums.swap(i, right_value_index);

        // Reverse nums[i+1]
        nums[i + 1..].reverse();

        false
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,1,2], vec![vec![1,1,2],vec![1,2,1],vec![2,1,1]])]
    #[case(vec![1,2,3], vec![vec![1,2,3],vec![1,3,2],vec![2,1,3],vec![2,3,1],vec![3,2,1],vec![3,1,2]])]
    fn case(#[case] nums: Vec<i32>, #[case] mut expected: Vec<Vec<i32>>) {
        let mut actual = Solution::permute_unique(nums);
        actual.sort_unstable();
        expected.sort_unstable();
        assert_eq!(actual, expected);
    }
}
