//! Solution for https://leetcode.com/problems/find-missing-and-repeated-values
//! 2965. Find Missing and Repeated Values

use std::collections::HashSet;

impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut a = None;
        let upper = (grid.len() * grid.len()) as i32;
        let mut map: HashSet<i32> = (1..=upper).collect();
        for row in grid.iter() {
            for cell in row {
                let existed = map.remove(cell);
                if !existed {
                    a = Some(*cell);
                }
            }
        }
        debug_assert_eq!(map.len(), 1);
        let b = map.iter().next().copied();
        vec![a.unwrap(), b.unwrap()]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,3],vec![2,2]], vec![2,4])]
    #[case(vec![vec![9,1,7],vec![8,9,2],vec![3,4,6]], vec![9,5])]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] expected: Vec<i32>) {
        let actual = Solution::find_missing_and_repeated_values(grid);
        assert_eq!(actual, expected);
    }
}
