impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        candidates.sort_unstable();
        let mut attempt = vec![];
        Self::_combination_sum2(&candidates, target, &mut result, &mut attempt);
        result
    }

    fn _combination_sum2(
        candidates: &[i32],
        target: i32,
        result: &mut Vec<Vec<i32>>,
        attempt: &mut Vec<i32>,
    ) {
        if target == 0 {
            result.push(attempt.clone());
            return;
        }
        let mut last_val = None;
        candidates
            .iter()
            .enumerate()
            .take_while(|(_, &val)| val <= target)
            .for_each(|(i, &val)| {
                if Some(val) != last_val {
                    last_val = Some(val);
                    attempt.push(val);
                    Self::_combination_sum2(&candidates[i + 1..], target - val, result, attempt);
                    attempt.pop();
                }
            })
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![10,1,2,7,6,1,5], 8,
        vec![
        vec![1,1,6],
        vec![1,2,5],
        vec![1,7],
        vec![2,6]
        ])]
    #[case(vec![2,5,2,1,2], 5,
        vec![
            vec![1,2,2],
            vec![5]
            ])]
    #[case(vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1], 27, vec![])]
    #[case(vec![
         1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
         1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
         1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
         1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1], 30,
        vec![vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
         1,1,1,1,1,1,1,1,1,1]])]
    fn case(
        #[case] candidates: Vec<i32>,
        #[case] target: i32,
        #[case] mut expected: Vec<Vec<i32>>,
    ) {
        let mut actual = Solution::combination_sum2(candidates, target);
        actual.iter_mut().for_each(|x| x.sort_unstable());
        expected.iter_mut().for_each(|x| x.sort_unstable());
        actual.sort_unstable();
        expected.sort_unstable();
        assert_eq!(actual, expected);
    }
}
