//! Solution for https://leetcode.com/problems/flatten-binary-tree-to-linked-list
//! 114. Flatten Binary Tree to Linked List

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
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(root) = root.as_mut() {
            // Disconnect children
            let mut left = root.borrow_mut().left.take();
            let mut right = root.borrow_mut().right.take();

            // Set next node to be current node
            let mut next = root.clone();

            // Flatten left child and move next node as needed
            Self::flatten(&mut left);

            // Attach root of left child on right
            next.borrow_mut().right = left;

            // Traverse to right most child
            loop {
                let Some(next_node) = next.borrow().right.clone() else {
                    break;
                };
                next = next_node;
            }

            // Attach original right child
            next.borrow_mut().right = right.clone();

            // Recurse on right child
            Self::flatten(&mut right);
        }
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;
use cargo_leet::TreeNode;

#[cfg(test)]
mod tests {
    use super::*;

    use cargo_leet::{TreeNode, TreeRoot};
    use rstest::rstest;

    #[rstest]
    #[case(TreeRoot::from("[1,2,5,3,4,null,6]").into(), TreeRoot::from("[1,null,2,null,3,null,4,null,5,null,6]").into())]
    #[case(TreeRoot::from("[]").into(), TreeRoot::from("[]").into())]
    #[case(TreeRoot::from("[0]").into(), TreeRoot::from("[0]").into())]
    fn case(
        #[case] mut root: Option<Rc<RefCell<TreeNode>>>,
        #[case] expected: Option<Rc<RefCell<TreeNode>>>,
    ) {
        Solution::flatten(&mut root);
        assert_eq!(root, expected);
    }
}
