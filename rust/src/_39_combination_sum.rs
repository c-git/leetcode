//! Solution for https://leetcode.com/problems/combination-sum
//! 39. Combination Sum

impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();
        let mut result = vec![];
        let mut partial = vec![];
        Self::combination_sum_(&candidates, target, &mut partial, &mut result);
        result
    }

    fn combination_sum_(
        candidates: &[i32],
        target: i32,
        partial: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if target == 0 {
            result.push(partial.clone());
            return;
        }
        if candidates.is_empty() {
            return;
        }
        if target < candidates[0] {
            return;
        }

        // Try another of the same
        partial.push(candidates[0]);
        Self::combination_sum_(candidates, target - candidates[0], partial, result);
        partial.pop();

        // Try without first
        Self::combination_sum_(&candidates[1..], target, partial, result);
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![2,3,6,7], 7, vec![vec![2,2,3],vec![7]])]
    #[case(vec![2,3,5], 8, vec![vec![2,2,2,2],vec![2,3,3],vec![3,5]])]
    #[case(vec![2], 1, vec![])]
    #[case(vec![3,5,8], 11, vec![vec![3,3,5],vec![3,8]])]
    #[case(vec![8,7,4,3], 11, vec![vec![8,3],vec![7,4],vec![4,4,3]])]
    fn case(
        #[case] candidates: Vec<i32>,
        #[case] target: i32,
        #[case] mut expected: Vec<Vec<i32>>,
    ) {
        let mut actual = Solution::combination_sum(candidates, target);
        actual.iter_mut().for_each(|x| x.sort_unstable());
        expected.iter_mut().for_each(|x| x.sort_unstable());
        actual.sort_unstable();
        expected.sort_unstable();
        assert_eq!(actual, expected);
    }
}
