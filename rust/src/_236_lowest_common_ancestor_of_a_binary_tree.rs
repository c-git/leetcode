//! Solution for https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree
//! 236. Lowest Common Ancestor of a Binary Tree

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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::lowest_common_ancestor_(root, p.unwrap().borrow().val, q.unwrap().borrow().val).2
    }

    /// Returns is_p_found, is_q_found, first_node_to_see_both_found
    fn lowest_common_ancestor_(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: i32,
        q: i32,
    ) -> (bool, bool, Option<Rc<RefCell<TreeNode>>>) {
        let Some(root) = root else {
            return Default::default();
        };
        let (left_is_p_found, left_is_q_found, left_found_at) =
            Self::lowest_common_ancestor_(root.borrow().left.clone(), p, q);
        let (right_is_p_found, right_is_q_found, right_found_at) =
            Self::lowest_common_ancestor_(root.borrow().right.clone(), p, q);
        if left_found_at.is_some() {
            return (true, true, left_found_at);
        }
        if right_found_at.is_some() {
            return (true, true, right_found_at);
        }
        let is_p_found = left_is_p_found || right_is_p_found || root.borrow().val == p;
        let is_q_found = left_is_q_found || right_is_q_found || root.borrow().val == q;
        if is_p_found && is_q_found {
            (is_p_found, is_q_found, Some(root))
        } else {
            (is_p_found, is_q_found, None)
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
    #[case(TreeRoot::from("[3,5,1,6,2,0,8,null,null,7,4]").into(), TreeRoot::from("[5]").into(), TreeRoot::from("[1]").into(), TreeRoot::from("[3]").into())]
    #[case(TreeRoot::from("[3,5,1,6,2,0,8,null,null,7,4]").into(), TreeRoot::from("[5]").into(), TreeRoot::from("[4]").into(), TreeRoot::from("[5]").into())]
    #[case(TreeRoot::from("[1,2]").into(), TreeRoot::from("[1]").into(), TreeRoot::from("[2]").into(), TreeRoot::from("[1]").into())]
    fn case(
        #[case] root: Option<Rc<RefCell<TreeNode>>>,
        #[case] p: Option<Rc<RefCell<TreeNode>>>,
        #[case] q: Option<Rc<RefCell<TreeNode>>>,
        #[case] expected: Option<Rc<RefCell<TreeNode>>>,
    ) {
        let actual = Solution::lowest_common_ancestor(root, p, q);
        assert_eq!(actual.unwrap().borrow().val, expected.unwrap().borrow().val);
    }
}
