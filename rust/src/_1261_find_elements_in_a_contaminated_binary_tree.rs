//! Solution for https://leetcode.com/problems/find-elements-in-a-contaminated-binary-tree
//! 1261. Find Elements in a Contaminated Binary Tree

use std::{cell::RefCell, collections::BTreeSet, rc::Rc};

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
pub struct FindElements {
    node_values: BTreeSet<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindElements {
    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let Some(root) = root else {
            return Self {
                node_values: Default::default(),
            };
        };
        root.borrow_mut().val = 0;
        let mut node_values = BTreeSet::new();
        let mut stack = vec![root];
        while let Some(node) = stack.pop() {
            let mut node = node.borrow_mut();
            let node_value = node.val;
            debug_assert_ne!(node_value, -1);
            node_values.insert(node_value);
            if let Some(left) = node.left.take() {
                left.borrow_mut().val = 2 * node_value + 1;
                stack.push(left);
            }
            if let Some(right) = node.right.take() {
                right.borrow_mut().val = 2 * node_value + 2;
                stack.push(right);
            }
        }
        Self { node_values }
    }

    pub fn find(&self, target: i32) -> bool {
        self.node_values.contains(&target)
    }
}

/*
 * Your FindElements object will be instantiated and called as such:
 * let obj = FindElements::new(root);
 * let ret_1: bool = obj.find(target);
 */
// << ---------------- Code below here is only for local use ---------------- >>

use cargo_leet::TreeNode;

#[cfg(test)]
mod tests {
    use cargo_leet::TreeRoot;

    use super::*;

    #[test]
    fn test1() {
        let find_elements = FindElements::new(TreeRoot::from("[-1,null,-1]").into());
        assert!(!find_elements.find(1)); // return False
        assert!(find_elements.find(2)); // return True
    }
}
