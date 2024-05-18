//! Solution for https://leetcode.com/problems/distribute-coins-in-binary-tree
//! 979. Distribute Coins in Binary Tree

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
struct Cost {
    /// Min number of moves that will distribute the subtree
    /// including bring any excess to the root or distributing the shortage
    /// but not moving coins to/from root from/to parent
    min_moves_needed: u32,

    /// Number of coins extra that have to be moved out of the subtree (negative means number to be added)
    coin_overage: i32,
}

impl Solution {
    pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let result = Self::distribute_coins_(root);
        debug_assert_eq!(result.coin_overage, 0, "should be exact at root");
        result.min_moves_needed as i32
    }

    fn distribute_coins_(root: Option<Rc<RefCell<TreeNode>>>) -> Cost {
        if let Some(node) = root {
            let node = node.borrow();
            let left_cost = Self::distribute_coins_(node.left.clone());
            let right_cost = Self::distribute_coins_(node.right.clone());
            let coin_overage = left_cost.coin_overage + right_cost.coin_overage + node.val - 1;
            // First we include the moves our children needed then we assume that
            // we must always have enough at the root (be that from our parent or taking from our children)
            // Then just add how many we have to take or give to our children which is how far they are from needing 0
            let min_moves_needed = left_cost.min_moves_needed
                + right_cost.min_moves_needed
                + left_cost.coin_overage.abs_diff(0)
                + right_cost.coin_overage.abs_diff(0);
            Cost {
                min_moves_needed,
                coin_overage,
            }
        } else {
            // Empty nodes are easy because no work is needed
            Cost {
                min_moves_needed: 0,
                coin_overage: 0,
            }
        }
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;
use cargo_leet::TreeNode;

#[cfg(test)]
mod tests {
    use super::*;
    use cargo_leet::TreeRoot;

    use rstest::rstest;

    #[rstest]
    #[case(TreeRoot::from("[3,0,0]").into(), 2)]
    #[case(TreeRoot::from("[0,3,0]").into(), 3)]
    #[case(TreeRoot::from("[1,1,1]").into(), 0)]
    #[case(TreeRoot::from("[1,0,2]").into(), 2)]
    #[case(TreeRoot::from("[1,2,0]").into(), 2)]
    #[case(TreeRoot::from("[2,1,0]").into(), 1)]
    fn case(#[case] root: Option<Rc<RefCell<TreeNode>>>, #[case] expected: i32) {
        let actual = Solution::distribute_coins(root);
        assert_eq!(actual, expected);
    }
}
