//! Solution for https://leetcode.com/problems/k-th-symbol-in-grammar
//! 779. K-th Symbol in Grammar

impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        let cap = 2usize.pow(n as u32);
        let mut last_row = Vec::with_capacity(cap);
        let mut new_row = Vec::with_capacity(cap);
        last_row.push(0);
        for _ in 2..=n {
            // println!("\n{last_row:?}");
            for element in last_row.iter() {
                match element {
                    0 => {
                        new_row.push(0);
                        new_row.push(1);
                    }
                    1 => {
                        new_row.push(1);
                        new_row.push(0);
                    }
                    _ => unreachable!(),
                }
            }
            last_row.clear();
            std::mem::swap(&mut last_row, &mut new_row);
        }
        last_row[k as usize - 1]
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
