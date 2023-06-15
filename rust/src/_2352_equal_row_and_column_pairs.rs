//! Solution for https://leetcode.com/problems/equal-row-and-column-pairs

use std::collections::HashMap;

enum EndStatus {
    NotEnd,
    Terminal { number_of_endings: i32 },
}

impl Default for EndStatus {
    fn default() -> Self {
        Self::NotEnd
    }
}

impl EndStatus {
    /// Returns `true` if the end status is [`IsTerminal`].
    ///
    /// [`IsTerminal`]: EndStatus::IsTerminal
    #[must_use]
    fn is_terminal(&self) -> bool {
        matches!(self, Self::Terminal { .. })
    }
}

#[derive(Default)]
struct Trie {
    children: HashMap<i32, Trie>,
    end_status: EndStatus,
}

impl Trie {
    fn add_row(&mut self, row: &[i32]) {
        let mut node = self;
        for &val in row {
            node = node.children.entry(val).or_default();
        }

        let new_count = match node.end_status {
            EndStatus::NotEnd => 1,
            EndStatus::Terminal { number_of_endings } => number_of_endings + 1,
        };

        node.end_status = EndStatus::Terminal {
            number_of_endings: new_count,
        }
    }

    /// Returns the number of matches
    fn lookup_column(&self, grid: &[Vec<i32>], column_num: usize) -> i32 {
        let mut node = Some(self);
        for row in grid.iter() {
            let value = row[column_num];
            if let Some(x) = node {
                node = x.children.get(&value);
            } else {
                break;
            }
        }

        if let Some(node) = node {
            match node.end_status {
                EndStatus::Terminal { number_of_endings } => number_of_endings,
                EndStatus::NotEnd => unreachable!(),
            }
        } else {
            0
        }
    }
}

impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        debug_assert_eq!(grid.len(), grid[0].len());
        let mut trie: Trie = Default::default();
        for row in grid.iter() {
            trie.add_row(row);
        }

        (0..grid.len()).map(|i| trie.lookup_column(&grid, i)).sum()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![3,2,1],vec![1,7,6],vec![2,7,7]], 1)]
    #[case(vec![vec![3,1,2,2],vec![1,4,4,5],vec![2,4,2,2],vec![2,4,2,2]], 3)]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::equal_pairs(grid);
        assert_eq!(actual, expected);
    }
}
