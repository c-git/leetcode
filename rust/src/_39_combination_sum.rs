impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        candidates.sort_unstable();
        for (i, &val) in candidates.iter().enumerate() {
            for multiplicity in 1.. {
                let prefix_sum = val * multiplicity;
                if prefix_sum <= target {
                    let mut attempt = vec![val; multiplicity as usize];
                    if Self::_combination_sum(
                        &candidates[i + 1..],
                        target - prefix_sum,
                        &mut attempt,
                    ) {
                        result.push(attempt);
                    }
                } else {
                    // No more solutions can be made from this value
                    break;
                }
            }
        }
        result
    }

    fn _combination_sum(candidates: &[i32], target: i32, attempt: &mut Vec<i32>) -> bool {
        debug_assert!(target >= 0);
        if target == 0 {
            return true;
        }
        if candidates.is_empty() {
            return false;
        }
        let first_element = candidates[0];

        if first_element <= target {
            attempt.push(first_element);
            Self::_combination_sum(candidates, target - first_element, attempt)
        } else {
            Self::_combination_sum(&candidates[1..], target, attempt)
        }
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
    fn case(
        #[case] candidates: Vec<i32>,
        #[case] target: i32,
        #[case] mut expected: Vec<Vec<i32>>,
    ) {
        let mut actual = Solution::combination_sum(candidates, target);
        actual.sort_unstable();
        expected.sort_unstable();
        assert_eq!(actual, expected);
    }
}
