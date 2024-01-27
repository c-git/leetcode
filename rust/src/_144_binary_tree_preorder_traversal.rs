//! Solution for https://leetcode.com/problems/binary-tree-preorder-traversal
//! 144. Binary Tree Preorder Traversal

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
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        Self::preorder_traversal_(&root, &mut result);
        result
    }

    fn preorder_traversal_(root: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(node) = root {
            let node = node.borrow();
            result.push(node.val);
            Self::preorder_traversal_(&node.left, result);
            Self::preorder_traversal_(&node.right, result);
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
    #[case(TreeRoot::from("[1,null,2,3]").into(), vec![1,2,3])]
    #[case(TreeRoot::from("[]").into(), vec![])]
    #[case(TreeRoot::from("[1]").into(),vec![1])]
    fn case(#[case] root: Option<Rc<RefCell<TreeNode>>>, #[case] expected: Vec<i32>) {
        let actual = Solution::preorder_traversal(root);
        assert_eq!(actual, expected);
    }
}
