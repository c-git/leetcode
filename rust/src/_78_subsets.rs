//! Solution for https://leetcode.com/problems/subsets
//! 78. Subsets

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // Based on editorial
        let mut result = vec![vec![]];
        for num in nums {
            let n = result.len();
            for i in 0..n {
                let mut next = result[i].clone();
                next.push(num);
                result.push(next);
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
    #[case(vec![1,2,3], vec![vec![],vec![1],vec![2],vec![1,2],vec![3],vec![1,3],vec![2,3],vec![1,2,3]])]
    #[case(vec![0], vec![vec![],vec![0]])]
    fn case(#[case] nums: Vec<i32>, #[case] expected: Vec<Vec<i32>>) {
        let actual = Solution::subsets(nums);
        assert_eq!(actual, expected);
    }
}
