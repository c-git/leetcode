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

use crate::helper::TreeNode;

pub struct Solution;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        root // TODO: Implement
    }
}

#[cfg(test)]
mod tests {
    use crate::helper::TreeRoot;

    use super::*;

    #[test]
    fn case1() {
        let root: TreeRoot = vec![4, 2, 7, 1, 3, 6, 9].into();
        let expected: TreeRoot = vec![4, 7, 2, 9, 6, 3, 1].into();

        let actual = Solution::invert_tree(root.into());
        assert_eq!(expected, actual.into());
    }

    #[test]
    fn case2() {
        let root: TreeRoot = vec![2, 1, 3].into();
        let expected: TreeRoot = vec![2, 3, 1].into();

        let actual = Solution::invert_tree(root.into());
        assert_eq!(expected, actual.into());
    }

    #[test]
    fn case3() {
        let root: TreeRoot = Vec::<i32>::new().into();
        let expected: TreeRoot = Vec::<i32>::new().into();

        let actual = Solution::invert_tree(root.into());
        assert_eq!(expected, actual.into());
    }
}
