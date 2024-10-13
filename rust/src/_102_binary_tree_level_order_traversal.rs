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
        let mut result = Vec::new();
        let Some(root) = root else {
            return result;
        };
        let mut queue = VecDeque::new();
        let mut level_nodes = vec![];
        let mut current_level = 0;
        queue.push_back((root, current_level));
        while let Some((node, node_level)) = queue.pop_front() {
            if node_level != current_level {
                result.push(level_nodes);
                level_nodes = Vec::new();
                current_level = node_level;
            }
            level_nodes.push(node.borrow().val);
            if let Some(left) = node.borrow_mut().left.take() {
                queue.push_back((left, node_level + 1));
            }
            if let Some(right) = node.borrow_mut().right.take() {
                queue.push_back((right, node_level + 1));
            }
        }
        if !level_nodes.is_empty() {
            result.push(level_nodes);
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
