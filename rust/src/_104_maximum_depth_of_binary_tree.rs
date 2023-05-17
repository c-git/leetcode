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
use std::cmp::max;
use std::rc::Rc;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            let left = root.borrow_mut().left.take();
            let right = root.borrow_mut().right.take();
            1 + max(Self::max_depth(left), Self::max_depth(right))
        } else {
            0
        }
    }
}

use crate::helper::TreeNode;
struct Solution;
#[cfg(test)]
mod tests {
    use crate::helper::TreeRoot;

    use super::*;

    #[test]
    fn case1() {
        let input: TreeRoot = "[3,9,20,null,null,15,7]".into();
        let expected = 3;
        let actual = Solution::max_depth(input.into());
        assert_eq!(actual, expected);
    }

    #[test]
    fn case2() {
        let input: TreeRoot = "[1,null,2]".into();
        let expected = 2;
        let actual = Solution::max_depth(input.into());
        assert_eq!(actual, expected);
    }
}
