impl Solution {
    // Source: https://leetcode.com/problems/combination-sum/solutions/3280398/rust-beats-100-memory-and-100-complexety-easy-recursive-sort-solution/
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        candidates.sort_unstable();
        Self::_combination_sum(&candidates, target, &mut result, vec![]);
        result
    }
    pub fn _combination_sum(
        candidates: &[i32],
        target: i32,
        result: &mut Vec<Vec<i32>>,
        attempt: Vec<i32>,
    ) {
        if target == 0 {
            return result.push(attempt);
        }
        candidates
            .iter()
            .enumerate()
            .take_while(|(_, &val)| val <= target)
            .for_each(|(i, &val)| {
                let mut new = attempt.clone();
                new.push(val);
                Self::_combination_sum(&candidates[i..], target - val, result, new)
            })
    }
}

struct Solution;
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
