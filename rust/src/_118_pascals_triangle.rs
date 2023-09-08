//! Solution for https://leetcode.com/problems/pascals-triangle
//! 118. Pascal's Triangle

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::with_capacity(num_rows as usize);
        result.push(vec![1]);
        for _ in 1..num_rows {
            let last = result.last().unwrap();
            let mut new_row = Vec::with_capacity(last.len() + 1);
            new_row.push(1);
            for i in 1..last.len() {
                new_row.push(last[i - 1] + last[i]);
            }
            new_row.push(1);
            result.push(new_row);
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
    #[case(5, vec![vec![1],vec![1,1],vec![1,2,1],vec![1,3,3,1],vec![1,4,6,4,1]])]
    #[case(1, vec![vec![1]])]
    fn case(#[case] num_rows: i32, #[case] expected: Vec<Vec<i32>>) {
        let actual = Solution::generate(num_rows);
        assert_eq!(actual, expected);
    }
}
