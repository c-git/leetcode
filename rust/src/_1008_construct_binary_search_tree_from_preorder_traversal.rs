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
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
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
        let input = vec![8, 5, 1, 7, 10, 12];
        let expected: TreeRoot = "[8, 5, 10, 1, 7, null, 12]".into();
        let actual = Solution::bst_from_preorder(input);
        assert_eq!(actual, expected.into());
    }

    #[test]
    fn case2() {
        let input = vec![1, 3];
        let expected: TreeRoot = "[1,null,3]".into();
        let actual = Solution::bst_from_preorder(input);
        assert_eq!(actual, expected.into());
    }
}
