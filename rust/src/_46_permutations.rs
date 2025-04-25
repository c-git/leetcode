//! Solution for https://leetcode.com/problems/permutations
//! 46. Permutations

impl Solution {
    /// Based on https://www.youtube.com/watch?v=FZe0UqISmUw
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![vec![]];

        // For each number insert it into each position in the previous solutions
        // Including at the end
        for num in nums {
            let mut new_result = vec![];
            for prev_result_val in result {
                for i in 0..=prev_result_val.len() {
                    let mut clone = prev_result_val.clone();
                    clone.insert(i, num);
                    new_result.push(clone);
                }
            }
            result = new_result;
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
