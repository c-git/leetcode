//! Solution for https://leetcode.com/problems/combinations
//! 77. Combinations

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut candidate: Vec<i32> = (1..=k).collect();
        let max_index = candidate.len() - 1;
        'outer: loop {
            result.push(candidate.clone());
            if candidate[max_index] < n {
                candidate[max_index] += 1;
            } else {
                for next_inc_idx in (0..max_index).rev() {
                    if candidate[next_inc_idx] < candidate[next_inc_idx + 1] - 1 {
                        candidate[next_inc_idx] += 1;
                        for reset_idx in next_inc_idx + 1..=max_index {
                            candidate[reset_idx] = candidate[reset_idx - 1] + 1;
                        }
                        continue 'outer;
                    }
                }
                break; // Unable to increment any values
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
    #[case(4, 2, vec![vec![1,2],vec![1,3],vec![1,4],vec![2,3],vec![2,4],vec![3,4]])]
    #[case(1, 1, vec![vec![1]])]
    fn case(#[case] n: i32, #[case] k: i32, #[case] expected: Vec<Vec<i32>>) {
        let actual = Solution::combine(n, k);
        assert_eq!(actual, expected);
    }
}
