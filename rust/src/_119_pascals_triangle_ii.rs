//! Solution for https://leetcode.com/problems/pascals-triangle-ii
//! 119. Pascal's Triangle II

use std::mem;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut last_row = Vec::with_capacity(row_index as usize + 1);
        let mut next_row = Vec::with_capacity(row_index as usize + 1);
        last_row.push(1);

        for _ in 1..=row_index {
            next_row.clear();
            next_row.push(1);
            for i in 1..last_row.len() {
                next_row.push(last_row[i - 1] + last_row[i]);
            }
            next_row.push(1);
            mem::swap(&mut last_row, &mut next_row);
        }
        last_row
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(3, vec![1,3,3,1])]
    #[case(0, vec![1])]
    #[case(1, vec![1,1])]
    fn case(#[case] row_index: i32, #[case] expected: Vec<i32>) {
        let actual = Solution::get_row(row_index);
        assert_eq!(actual, expected);
    }
}
