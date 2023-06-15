//! Solution for https://leetcode.com/problems/minimum-distance-between-bst-nodes
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
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = i32::MAX;
        if let Some(node) = root.as_ref() {
            let node = node.borrow();
            if let Some(left) = &node.left {
                result = result
                    .min(node.val - left.borrow().val)
                    .min(Self::min_diff_in_bst(node.left.clone()));
            }
            if let Some(right) = &node.right {
                result = result
                    .min(right.borrow().val - node.val)
                    .min(Self::min_diff_in_bst(node.right.clone()));
            }
        }
        result
    }
}

struct Solution;
use cargo_leet::TreeNode;

#[cfg(test)]
mod tests {
    use super::*;
    use cargo_leet::TreeRoot;

    use rstest::rstest;

    #[rstest]
    #[case(TreeRoot::from("[4,2,6,1,3]").into(), 1)]
    #[case(TreeRoot::from("[1,0,48,null,null,12,49]").into(), 1)]
    fn case(#[case] root: Option<Rc<RefCell<TreeNode>>>, #[case] expected: i32) {
        let actual = Solution::min_diff_in_bst(root);
        assert_eq!(actual, expected);
    }
}
