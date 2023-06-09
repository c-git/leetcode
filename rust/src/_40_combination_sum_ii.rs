use std::collections::HashSet;

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = HashSet::new();
        candidates.sort_unstable();
        Self::_combination_sum2(
            &candidates,
            target,
            candidates.iter().sum(),
            &mut result,
            vec![],
        );
        result.into_iter().collect()
    }

    fn _combination_sum2(
        candidates: &[i32],
        target: i32,
        remaining_sum: i32,
        result: &mut HashSet<Vec<i32>>,
        attempt: Vec<i32>,
    ) {
        if target == 0 {
            result.insert(attempt);
            return;
        }
        if target > remaining_sum {
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
                    let mut new = attempt.clone();
                    new.push(val);
                    Self::_combination_sum2(
                        &candidates[i + 1..],
                        target - val,
                        remaining_sum - val,
                        result,
                        new,
                    )
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
