//! Solution for https://leetcode.com/problems/number-of-provinces
//! 547. Number of Provinces

use std::collections::HashSet;

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut uf = UnionFind::new(is_connected.len());
        for (row_idx, row) in is_connected.iter().enumerate() {
            for (col_idx, &cell) in row.iter().enumerate() {
                if cell == 1 {
                    uf.merge(row_idx, col_idx);
                }
            }
        }
        let mut seen = HashSet::new();
        for i in 0..is_connected.len() {
            seen.insert(uf.find(i));
        }
        seen.len() as i32
    }
}

struct UnionFind {
    nodes: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        Self {
            nodes: (0..size).collect(),
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.nodes[x] != x {
            self.nodes[x] = self.find(self.nodes[x]);
        }
        self.nodes[x]
    }

    fn merge(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        self.nodes[root_x] = root_y;
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,1,0],vec![1,1,0],vec![0,0,1]], 2)]
    #[case(vec![vec![1,0,0],vec![0,1,0],vec![0,0,1]], 3)]
    fn case(#[case] is_connected: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::find_circle_num(is_connected);
        assert_eq!(actual, expected);
    }
}
