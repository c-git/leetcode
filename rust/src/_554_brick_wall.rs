//! Solution for https://leetcode.com/problems/brick-wall
//! 554. Brick Wall

use std::collections::HashMap;

impl Solution {
    pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        let mut highest_aligned_endpoints = 0;
        let n = wall.len();
        let mut endpoints: HashMap<i32, usize> = HashMap::new();
        for row in wall {
            let mut endpoint = 0;
            let brick_to_use = row.len() - 1;
            for brick in row[..brick_to_use].iter().copied() {
                endpoint += brick;
                let entry = endpoints.entry(endpoint).or_default();
                *entry += 1;
                highest_aligned_endpoints = highest_aligned_endpoints.max(*entry);
            }
        }
        (n - highest_aligned_endpoints) as _
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,2,2,1],vec![3,1,2],vec![1,3,2],vec![2,4],vec![3,1,2],vec![1,3,1,1]], 2)]
    #[case(vec![vec![1],vec![1],vec![1]], 3)]
    fn case(#[case] wall: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::least_bricks(wall);
        assert_eq!(actual, expected);
    }
}
