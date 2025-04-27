//! Solution for https://leetcode.com/problems/validate-binary-search-tree
//! 98. Validate Binary Search Tree

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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let Some(root) = root else {
            return true;
        };
        Self::is_valid_bst_(root, i32::MIN, i32::MAX)
    }

    fn is_valid_bst_(root: Rc<RefCell<TreeNode>>, low: i32, high: i32) -> bool {
        let mut root = root.borrow_mut();
        if !(low..=high).contains(&root.val) {
            return false;
        }
        if let Some(left) = root.left.take() {
            if !Self::is_valid_bst_(left, low, root.val.saturating_sub(1)) {
                return false;
            }
        }
        if let Some(left) = root.right.take() {
            let Some(new_low) = root.val.checked_add(1) else {
                return false;
            };
            if !Self::is_valid_bst_(left, new_low, high) {
                return false;
            }
        }
        true
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
    #[case(TreeRoot::from("[2,1,3]").into(), true)]
    #[case(TreeRoot::from("[5,1,4,null,null,3,6]").into(), false)]
    #[case(TreeRoot::from("[2,2,2]").into(), false)]
    fn case(#[case] root: Option<Rc<RefCell<TreeNode>>>, #[case] expected: bool) {
        let actual = Solution::is_valid_bst(root);
        assert_eq!(actual, expected);
    }
}
