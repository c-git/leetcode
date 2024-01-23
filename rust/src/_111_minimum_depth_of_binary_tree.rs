//! Solution for https://leetcode.com/problems/minimum-depth-of-binary-tree
//! 111. Minimum Depth of Binary Tree

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
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            1 + match (node.left.is_some(), node.right.is_some()) {
                (true, true) => {
                    Self::min_depth(node.left.clone()).min(Self::min_depth(node.right.clone()))
                }
                (true, false) => Self::min_depth(node.left.clone()),
                (false, true) => Self::min_depth(node.right.clone()),
                (false, false) => 0,
            }
        } else {
            0
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
    #[case(TreeRoot::from("[3,9,20,null,null,15,7]").into(), 2)]
    #[case(TreeRoot::from("[2,null,3,null,4,null,5,null,6]").into(), 5)]
    fn case(#[case] root: Option<Rc<RefCell<TreeNode>>>, #[case] expected: i32) {
        let actual = Solution::min_depth(root);
        assert_eq!(actual, expected);
    }
}
