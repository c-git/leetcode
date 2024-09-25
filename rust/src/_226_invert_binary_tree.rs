//! Solution for https://leetcode.com/problems/invert-binary-tree
//! 226. Invert Binary Tree

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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let root = root?;
        let mut node = root.borrow_mut();
        let temp = node.left.clone();
        node.left = Self::invert_tree(node.right.clone());
        node.right = Self::invert_tree(temp);
        drop(node); // Drop to release borrow
        Some(root)
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
    #[case(TreeRoot::from("[4,2,7,1,3,6,9]").into(), TreeRoot::from("[4,7,2,9,6,3,1]").into())]
    #[case(TreeRoot::from("[2,1,3]").into(), TreeRoot::from("[2,3,1]").into())]
    #[case(TreeRoot::from("[]").into(), TreeRoot::from("[]").into())]
    fn case(
        #[case] root: Option<Rc<RefCell<TreeNode>>>,
        #[case] expected: Option<Rc<RefCell<TreeNode>>>,
    ) {
        let actual = Solution::invert_tree(root);
        assert_eq!(actual, expected);
    }
}
