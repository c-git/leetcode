//! Solution for https://leetcode.com/problems/k-th-symbol-in-grammar
//! 779. K-th Symbol in Grammar

impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        let cap = 2usize.pow(n as u32);
        let mut row = Vec::with_capacity(cap);
        row.push(0);
        for _ in 2..=n {
            let curr_size = row.len();
            for i in 0..curr_size {
                match row[i] {
                    0 => {
                        row.push(1);
                    }
                    1 => {
                        row.push(0);
                    }
                    _ => unreachable!(),
                }
            }
        }
        row[k as usize - 1]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(1, 1, 0)]
    #[case(2, 1, 0)]
    #[case(2, 2, 1)]
    #[case(30, 536870912, 1)]
    fn case(#[case] n: i32, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::kth_grammar(n, k);
        assert_eq!(actual, expected);
    }
}
