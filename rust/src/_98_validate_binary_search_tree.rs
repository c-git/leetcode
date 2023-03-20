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

struct Solution;

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

    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_valid_bst_in_range(root, None, None)
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

    #[test]
    fn case3() {
        let input: TreeRoot = "[5,4,6,null,null,3,7]".into();
        let expected = false;

        let actual = Solution::is_valid_bst(input.into());
        assert_eq!(actual, expected);
    }

    #[test]
    fn case4() {
        let input: TreeRoot = "[2,2,2]".into();
        let expected = false;

        let actual = Solution::is_valid_bst(input.into());
        assert_eq!(actual, expected);
    }
}
