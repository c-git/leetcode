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
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root) = root {
            let root = root.borrow();
            if let Some(left) = &root.left {
                let left = left.borrow();
                // Based on question equal is not allowed
                if root.val <= left.val {
                    return false;
                }
            }
            if let Some(right) = &root.right {
                let right = right.borrow();
                // Based on question equal is not allowed
                if root.val >= right.val {
                    return false;
                }
            }
            Self::is_valid_bst(root.left.clone()) && Self::is_valid_bst(root.right.clone())
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::helper::TreeRoot;

    use super::*;

    #[test]
    fn case1() {
        let input: TreeRoot = "[2,1,3]".into();
        let expected = true;

        let actual = Solution::is_valid_bst(input.into());
        assert_eq!(actual, expected);
    }

    #[test]
    fn case2() {
        let input: TreeRoot = "[5,1,4,null,null,3,6]".into();
        let expected = false;

        let actual = Solution::is_valid_bst(input.into());
        assert_eq!(actual, expected);
    }
}
