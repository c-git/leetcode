use std::rc::Rc;
use std::{cell::RefCell, mem};

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
use crate::helper::TreeNode;
impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        // Based on solution from https://leetcode.com/problems/recover-binary-search-tree/solutions/1964742/rust-implementation-with-and-without-additional-memory/?languageTags=rust

        // TLDR: Find first out of place value in an in place traversal and find the place where it must go to not be wrong compared to the node after it
        let mut current_node = root.clone();
        let mut previous_node: Option<Rc<RefCell<TreeNode>>> = None;
        let mut left_swap_node: Option<Rc<RefCell<TreeNode>>> = None;
        let mut right_swap_node: Option<Rc<RefCell<TreeNode>>> = None;
        let mut stack = vec![];

        while !stack.is_empty() || current_node.is_some() {
            // Go to leftmost node if on a new node (like root or first time visiting a right child)
            while current_node.is_some() {
                stack.push(current_node.clone());
                current_node = current_node.clone().unwrap().borrow().left.clone();
            }
            current_node = stack
                .pop()
                .expect("Either stack already had values or current_node at start was added to it");

            // Perform action
            if left_swap_node.is_none() {
                // Looking for left side of swap
                if previous_node.is_some()
                    && previous_node.as_ref().unwrap().borrow().val
                        > current_node.as_ref().unwrap().borrow().val
                {
                    left_swap_node = previous_node;
                }
            } else {
                // Looking for where the left node needs to go to
                if left_swap_node.as_ref().unwrap().borrow().val
                    < current_node.as_ref().unwrap().borrow().val
                {
                    // Needs to go in previous position
                    right_swap_node = previous_node.clone();
                    break;
                }
            }

            // Go to next node
            previous_node = current_node;
            current_node = previous_node.as_ref().unwrap().borrow().right.clone();
        }

        // Handle case where it goes at the end of the in order traversal
        if right_swap_node.is_none() {
            right_swap_node = previous_node;
        }

        match (left_swap_node, right_swap_node) {
            (Some(a), Some(b)) => mem::swap(&mut a.borrow_mut().val, &mut b.borrow_mut().val),
            (_, _) => unreachable!(
                "Question asserts that there will always be a swap that needs to be made"
            ),
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::helper::TreeRoot;

    use super::*;

    #[test]
    fn case1() {
        let input: TreeRoot = "[1,3,null,null,2]".into();
        let mut input: Option<Rc<RefCell<TreeNode>>> = input.into();
        let expected: TreeRoot = "[3,1,null,null,2]".into();
        Solution::recover_tree(&mut input);
        assert_eq!(expected, input.into());
    }

    #[test]
    fn case2() {
        let input: TreeRoot = "[3,1,4,null,null,2]".into();
        let mut input: Option<Rc<RefCell<TreeNode>>> = input.into();
        let expected: TreeRoot = "[2,1,4,null,null,3]".into();
        Solution::recover_tree(&mut input);
        assert_eq!(expected, input.into());
    }

    #[test]
    fn case3() {
        let input: TreeRoot = "[2,3,1]".into();
        let mut input: Option<Rc<RefCell<TreeNode>>> = input.into();
        let expected: TreeRoot = "[2,1,3]".into();
        Solution::recover_tree(&mut input);
        assert_eq!(expected, input.into());
    }

    #[test]
    fn case4() {
        // To test break statement
        let input: TreeRoot = "[2,3,1,null,null,null,5]".into();
        let mut input: Option<Rc<RefCell<TreeNode>>> = input.into();
        let expected: TreeRoot = "[2,1,3,null,null,null,5]".into();
        Solution::recover_tree(&mut input);
        assert_eq!(expected, input.into());
    }
}
