//! Solution for https://leetcode.com/problems/evaluate-boolean-binary-tree
//! 2331. Evaluate Boolean Binary Tree

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
impl Solution {
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let Some(node) = root else {
            unreachable!("expected a full binary tree with at least 1 node. This function should never be called on an empty branch")
        };

        let mut node = node.borrow_mut();
        match node.val {
            0 => false,
            1 => true,
            2 => Self::evaluate_tree(node.left.take()) || Self::evaluate_tree(node.right.take()),
            3 => Self::evaluate_tree(node.left.take()) && Self::evaluate_tree(node.right.take()),
            _ => unreachable!("constraint says only 0-3 are allowed"),
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
    #[case(TreeRoot::from("[2,1,3,null,null,0,1]").into(), true)]
    #[case(TreeRoot::from("[0]").into(), false)]
    fn case(#[case] root: Option<Rc<RefCell<TreeNode>>>, #[case] expected: bool) {
        let actual = Solution::evaluate_tree(root);
        assert_eq!(actual, expected);
    }
}
