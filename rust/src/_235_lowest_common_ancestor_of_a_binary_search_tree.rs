//! Solution for https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree
//! 235. Lowest Common Ancestor of a Binary Search Tree

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
use std::cmp::Ordering;
use std::rc::Rc;
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let (left_cmp, right_cmp) = {
            // Blocked needed to drop root_node (Uncertain why this caused problems on Leetcode didn't give problems on local)
            let root_node = root.as_ref()?.borrow();
            let left_cmp = p.as_ref()?.borrow().val.cmp(&root_node.val);
            let right_cmp = q.as_ref()?.borrow().val.cmp(&root_node.val);
            (left_cmp, right_cmp)
        };
        match (left_cmp, right_cmp) {
            (Ordering::Less, Ordering::Less) => {
                Self::lowest_common_ancestor(root?.borrow_mut().left.take(), p, q)
            }
            (Ordering::Greater, Ordering::Greater) => {
                Self::lowest_common_ancestor(root?.borrow_mut().right.take(), p, q)
            }
            _ => root,
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
    #[case(TreeRoot::from("[6,2,8,0,4,7,9,null,null,3,5]").into(), TreeRoot::from("[2]").into(), TreeRoot::from("[8]").into(), TreeRoot::from("[6]").into())]
    #[case(TreeRoot::from("[6,2,8,0,4,7,9,null,null,3,5]").into(), TreeRoot::from("[2]").into(), TreeRoot::from("[4]").into(), TreeRoot::from("[2]").into())]
    #[case(TreeRoot::from("[2,1]").into(), TreeRoot::from("[2]").into(), TreeRoot::from("[1]").into(), TreeRoot::from("[2]").into())]
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
