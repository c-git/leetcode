//! Solution for https://leetcode.com/problems/binary-tree-maximum-path-sum
//! 124. Binary Tree Maximum Path Sum

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
    /// Based on https://www.youtube.com/watch?v=cfn-G-7vVlo
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::max_path_sum_(root).0
    }

    /// Returns the best path seen and the best path extension value possible if using this subtree
    pub fn max_path_sum_(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        let Some(root) = root else {
            return (i32::MIN, i32::MIN);
        };

        // Calculate values for trees rooted at children
        let (left_best, left_extension_value) = Self::max_path_sum_(root.borrow_mut().left.take());
        let (right_best, right_extension_value) =
            Self::max_path_sum_(root.borrow_mut().right.take());

        // Calculate path including this node and its children of this node
        let path_with_children =
            root.borrow().val + left_extension_value.max(0) + right_extension_value.max(0);

        // Compose sub results into final result for this subtree
        let result_best_path = left_best.max(right_best).max(path_with_children);
        let result_path_extension =
            root.borrow().val + left_extension_value.max(right_extension_value).max(0);

        (result_best_path, result_path_extension)
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
    #[case(TreeRoot::from("[1,2,3]").into(), 6)]
    #[case(TreeRoot::from("[-10,9,20,null,null,15,7]").into(), 42)]
    fn case(#[case] root: Option<Rc<RefCell<TreeNode>>>, #[case] expected: i32) {
        let actual = Solution::max_path_sum(root);
        assert_eq!(actual, expected);
    }
}
