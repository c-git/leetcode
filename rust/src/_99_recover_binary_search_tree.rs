use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

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
use crate::helper::TreeNode;

impl Solution {
    fn is_valid_bst_in_range(
        root: Option<Rc<RefCell<TreeNode>>>,
        min: Option<i32>,
        max: Option<i32>,
    ) -> bool {
        // Based on Fasil's idea
        if let Some(root) = root {
            let root = root.borrow();
            if let Some(min) = min {
                if root.val <= min {
                    return false;
                }
            }
            if let Some(max) = max {
                if root.val >= max {
                    return false;
                }
            }
            Self::is_valid_bst_in_range(root.left.clone(), min, Some(root.val))
                && Self::is_valid_bst_in_range(root.right.clone(), Some(root.val), max)
        } else {
            true
        }
    }

    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        Self::is_valid_bst_in_range(root.clone(), None, None);
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::helper::TreeRoot;

    use super::*;

    #[test]
    fn case1() {
        let input: TreeRoot = "[1,3,null,null,2]".into();
        let mut input: Option<Rc<RefCell<TreeNode>>> = input.into();
        let expected: TreeRoot = "[3,1,null,null,2]".into();
        Solution::recover_tree(&mut input);
        assert_eq!(expected, input.into());
    }

    #[test]
    fn case2() {
        let input: TreeRoot = "[3,1,4,null,null,2]".into();
        let mut input: Option<Rc<RefCell<TreeNode>>> = input.into();
        let expected: TreeRoot = "[2,1,4,null,null,3]".into();
        Solution::recover_tree(&mut input);
        assert_eq!(expected, input.into());
    }
}
