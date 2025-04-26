//! Solution for https://leetcode.com/problems/binary-tree-level-order-traversal
//! 102. Binary Tree Level Order Traversal

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
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let Some(root) = root else {
            return vec![];
        };

        let mut result = vec![];
        let mut queue = VecDeque::new();
        queue.push_back((root, 0));
        while let Some((node, level)) = queue.pop_front() {
            if level >= result.len() {
                // Extend to store new level
                result.push(vec![]);
                debug_assert!(level < result.len());
            }

            result[level].push(node.borrow().val);
            if let Some(left) = node.borrow_mut().left.take() {
                queue.push_back((left, level + 1));
            }
            if let Some(right) = node.borrow_mut().right.take() {
                queue.push_back((right, level + 1));
            }
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
    #[case(TreeRoot::from("[3,9,20,null,null,15,7]").into(), vec![vec![3],vec![9,20],vec![15,7]])]
    #[case(TreeRoot::from("[1]").into(), vec![vec![1]])]
    #[case(TreeRoot::from("[]").into(), vec![])]
    fn case(#[case] root: Option<Rc<RefCell<TreeNode>>>, #[case] expected: Vec<Vec<i32>>) {
        let actual = Solution::level_order(root);
        assert_eq!(actual, expected);
    }
}
