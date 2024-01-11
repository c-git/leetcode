//! Solution for https://leetcode.com/problems/maximum-difference-between-node-and-ancestor
//! 1026. Maximum Difference Between Node and Ancestor

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
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let node = root.expect("Constraint guarantees at least 2 nodes");
        let node = node.borrow();
        Self::max_ancestor_diff_(&node.left, node.val, node.val).max(Self::max_ancestor_diff_(
            &node.right,
            node.val,
            node.val,
        ))
    }

    fn max_ancestor_diff_(
        root: &Option<Rc<RefCell<TreeNode>>>,
        mut min_ancestor_val: i32,
        mut max_ancestor_val: i32,
    ) -> i32 {
        let mut result = 0;
        if let Some(node) = root {
            let node = node.borrow();
            result = result
                .max((node.val - min_ancestor_val).abs())
                .max((node.val - max_ancestor_val).abs());
            min_ancestor_val = min_ancestor_val.min(node.val);
            max_ancestor_val = max_ancestor_val.max(node.val);
            result = result.max(Self::max_ancestor_diff_(
                &node.left,
                min_ancestor_val,
                max_ancestor_val,
            ));
            result = result.max(Self::max_ancestor_diff_(
                &node.right,
                min_ancestor_val,
                max_ancestor_val,
            ));
        }
        result
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
    #[case(TreeRoot::from("[8,3,10,1,6,null,14,null,null,4,7,13]").into(), 7)]
    #[case(TreeRoot::from("[1,null,2,null,0,3]").into(), 3)]
    fn case(#[case] root: Option<Rc<RefCell<TreeNode>>>, #[case] expected: i32) {
        let actual = Solution::max_ancestor_diff(root);
        assert_eq!(actual, expected);
    }
}
