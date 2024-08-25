//! Solution for https://leetcode.com/problems/binary-tree-postorder-traversal
//! 145. Binary Tree Postorder Traversal

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
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        let Some(node) = root else {
            return result;
        };
        let mut stack = vec![node];
        while let Some(node) = stack.pop() {
            let node = node.borrow();
            result.push(node.val);
            if let Some(left) = node.left.clone() {
                stack.push(left);
            }
            if let Some(right) = node.right.clone() {
                stack.push(right);
            }
        }
        result.reverse();
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
    #[case(TreeRoot::from("[1,null,2,3]").into(), vec![3,2,1])]
    #[case(TreeRoot::from("[]").into(), vec![])]
    #[case(TreeRoot::from("[1]").into(), vec![1])]
    fn case(#[case] root: Option<Rc<RefCell<TreeNode>>>, #[case] expected: Vec<i32>) {
        let actual = Solution::postorder_traversal(root);
        assert_eq!(actual, expected);
    }
}
