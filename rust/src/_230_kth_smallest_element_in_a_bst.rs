//! Solution for https://leetcode.com/problems/kth-smallest-element-in-a-bst
//! 230. Kth Smallest Element in a BST

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
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        Self::kth_smallest_(&root, k as u16).1
    }
    fn kth_smallest_(root: &Option<Rc<RefCell<TreeNode>>>, k: u16) -> (u16, i32) {
        let Some(root) = root else {
            return (k, i32::default());
        };
        let (k, left_result) = Self::kth_smallest_(&root.borrow().left, k);
        if k == 0 {
            (0, left_result)
        } else if k == 1 {
            (0, root.borrow().val)
        } else {
            Self::kth_smallest_(&root.borrow().right, k - 1)
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
    #[case(TreeRoot::from("[3,1,4,null,2]").into(), 1, 1)]
    #[case(TreeRoot::from("[5,3,6,2,4,null,null,1]").into(), 3, 3)]
    fn case(#[case] root: Option<Rc<RefCell<TreeNode>>>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::kth_smallest(root, k);
        assert_eq!(actual, expected);
    }
}
