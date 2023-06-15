//! Solution for https://leetcode.com/problems/minimum-distance-between-bst-nodes
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
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = i32::MAX;
        let mut last_value = None;
        let mut current_node = root;
        let mut stack = vec![];
        while !stack.is_empty() || current_node.is_some() {
            // Go to leftmost node
            while current_node.is_some() {
                stack.push(current_node.clone());
                current_node = current_node.unwrap().as_ref().borrow().left.clone();
            }
            current_node = stack
                .pop()
                .expect("Only Some(_) should have been added to stack");

            // Perform action
            let curr_val = current_node.as_ref().unwrap().borrow().val;
            if let Some(last_val) = last_value {
                result = result.min(curr_val - last_val);
            }
            last_value = Some(curr_val);

            // Check if this node has a right subtree
            current_node = current_node.unwrap().as_ref().borrow().right.clone();
        }
        result
    }
}

struct Solution;
use cargo_leet::TreeNode;

#[cfg(test)]
mod tests {
    use super::*;
    use cargo_leet::TreeRoot;

    use rstest::rstest;

    #[rstest]
    #[case(TreeRoot::from("[4,2,6,1,3]").into(), 1)]
    #[case(TreeRoot::from("[1,0,48,null,null,12,49]").into(), 1)]
    fn case(#[case] root: Option<Rc<RefCell<TreeNode>>>, #[case] expected: i32) {
        let actual = Solution::min_diff_in_bst(root);
        assert_eq!(actual, expected);
    }
}
