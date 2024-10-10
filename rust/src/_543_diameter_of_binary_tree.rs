//! Solution for https://leetcode.com/problems/diameter-of-binary-tree
//! 543. Diameter of Binary Tree

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

struct TreeInfo {
    height: usize,
    diameter: usize,
}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::get_tree_info(&root).diameter as _
    }

    fn get_tree_info(root: &Option<Rc<RefCell<TreeNode>>>) -> TreeInfo {
        let Some(root) = root else {
            return TreeInfo {
                height: 0,
                diameter: 0,
            };
        };
        let left = Self::get_tree_info(&root.borrow().left);
        let right = Self::get_tree_info(&root.borrow().right);
        TreeInfo {
            height: left.height.max(right.height) + 1,
            diameter: left
                .diameter
                .max(right.diameter)
                .max(left.height + right.height),
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
    #[case(TreeRoot::from("[1,2,3,4,5]").into(), 3)]
    #[case(TreeRoot::from("[1,2]").into(), 1)]
    fn case(#[case] root: Option<Rc<RefCell<TreeNode>>>, #[case] expected: i32) {
        let actual = Solution::diameter_of_binary_tree(root);
        assert_eq!(actual, expected);
    }
}
