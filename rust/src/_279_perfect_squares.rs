//! Solution for https://leetcode.com/problems/perfect-squares
//! 279. Perfect Squares

use std::collections::HashMap;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let perfect_squares: Vec<i32> = (1..=n)
            .map_while(|x| {
                let y = x * x;
                if y <= n {
                    Some(y)
                } else {
                    None
                }
            })
            .collect();

        let mut memo = HashMap::new();
        Self::num_squares_(n, &perfect_squares, &mut memo)
            .expect("should always have an answer if n is 1 or more")
    }

    fn num_squares_(
        n: i32,
        perfect_squares: &[i32],
        memo: &mut HashMap<(i32, usize), Option<i32>>,
    ) -> Option<i32> {
        debug_assert!(n >= 0);
        if perfect_squares.is_empty() {
            return None;
        }
        if n <= 1 {
            // 1 number needed to make 1 and 0 numbers needed to make 0
            return Some(n);
        }
        let key = (n, perfect_squares.len());
        if let Some(result) = memo.get(&key) {
            return *result;
        }

        let largest_index_that_fits = perfect_squares
            .iter()
            .enumerate()
            .rev()
            .find_map(|(i, x)| if x <= &n { Some(i) } else { None })
            .expect("at least 1 should always fit");

        let use_last = Self::num_squares_(
            n - perfect_squares[largest_index_that_fits],
            &perfect_squares[..=largest_index_that_fits],
            memo,
        );
        let skip_last = Self::num_squares_(n, &perfect_squares[..largest_index_that_fits], memo);
        let result = match (use_last, skip_last) {
            (None, _) => {
                unreachable!("since 1 is in the list we must always be able to use that")
            }
            (Some(x), None) => Some(x + 1),
            (Some(x), Some(y)) => Some(y.min(x + 1)),
        };

        memo.insert(key, result);
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
    #[case(12, 3)]
    #[case(13, 2)]
    #[case(1, 1)]
    fn case(#[case] n: i32, #[case] expected: i32) {
        let actual = Solution::num_squares(n);
        assert_eq!(actual, expected);
    }
}
