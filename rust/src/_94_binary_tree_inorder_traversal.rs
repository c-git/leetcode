//! Solution for https://leetcode.com/problems/binary-tree-inorder-traversal
//! 94. Binary Tree Inorder Traversal

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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        Self::inorder_traversal_(root, &mut result);
        result
    }

    fn inorder_traversal_(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        let Some(root) = root else {
            return;
        };
        Self::inorder_traversal_(root.borrow_mut().left.take(), result);
        result.push(root.borrow().val);
        Self::inorder_traversal_(root.borrow_mut().right.take(), result);
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
    #[case(TreeRoot::from("[1,null,2,3]").into(), vec![1,3,2])]
    #[case(TreeRoot::from("[1,2,3,4,5,null,8,null,null,6,7,9]").into(), vec![4,2,6,5,7,1,3,9,8])]
    #[case(TreeRoot::from("[]").into(), vec![])]
    #[case(TreeRoot::from("[1]").into(), vec![1])]
    fn case(#[case] root: Option<Rc<RefCell<TreeNode>>>, #[case] expected: Vec<i32>) {
        let actual = Solution::inorder_traversal(root);
        assert_eq!(actual, expected);
    }
}
