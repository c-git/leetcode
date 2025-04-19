//! Solution for https://leetcode.com/problems/same-tree
//! 100. Same Tree

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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (None, None) => true,
            (None, Some(_)) | (Some(_), None) => false,
            (Some(x), Some(y)) => {
                x.borrow().val == y.borrow().val
                    && Self::is_same_tree(x.borrow().left.clone(), y.borrow().left.clone())
                    && Self::is_same_tree(x.borrow().right.clone(), y.borrow().right.clone())
            }
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
    #[case(TreeRoot::from("[1,2,3]").into(), TreeRoot::from("[1,2,3]").into(), true)]
    #[case(TreeRoot::from("[1,2]").into(), TreeRoot::from("[1,null,2]").into(), false)]
    #[case(TreeRoot::from("[1,2,1]").into(), TreeRoot::from("[1,1,2]").into(), false)]
    fn case(
        #[case] p: Option<Rc<RefCell<TreeNode>>>,
        #[case] q: Option<Rc<RefCell<TreeNode>>>,
        #[case] expected: bool,
    ) {
        let actual = Solution::is_same_tree(p, q);
        assert_eq!(actual, expected);
    }
}
