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

use crate::helper::TreeNode;

type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn recover_tree(root: &mut Node) {
        // Source: Based on Sample 0ms solution after completing problem. Mostly took better ways to express what I wanted to happen

        // TLDR: Find first out of place value in an in place traversal
        // and capture this node plus the adjacent node incase this is the other node (like if this is the end of the traversal or they just happen to be next to each other)
        // then continue and see if there is another node out of place and if yes that will be the one to switch with instead so we can stop searching at that point based on
        // constraints in the question

        let mut first = Node::None; // First node to be swapped
        let mut second = Node::None; // Second node to be swapped
        let mut previous_node = Node::None;

        let mut current_node = root.as_ref().map(Rc::clone);
        let mut stack = vec![];
        while current_node.is_some() || !stack.is_empty() {
            if let Some(node) = current_node {
                // Keep going left until we fall off the bottom left edge of this subtree
                stack.push(Rc::clone(&node));
                current_node = node.borrow().left.as_ref().map(Rc::clone);
            } else if let Some(node) = &stack.pop() {
                // Take last value put onto stack and perform logic here
                let node_val = node.borrow().val;

                if let Some(pre_node) = &previous_node {
                    let pre_val = pre_node.borrow().val;
                    if pre_val > node_val {
                        if first.is_none() {
                            first = Some(Rc::clone(pre_node));
                            second = Some(Rc::clone(node)); // Done here be the adjacent node might be the one to switch with
                        } else {
                            second = Some(Rc::clone(node));
                            break; // Based on question there is exactly one switch to be made, no point checking the rest of the nodes
                        }
                    }
                }

                previous_node = Some(Rc::clone(node));
                current_node = node.borrow().right.as_ref().map(Rc::clone); // Setup for next iteration, will go to bottom left of this tree if exists else it will go up the stack
            }
        }

        if let (Some(first), Some(second)) = (first, second) {
            std::mem::swap(&mut first.borrow_mut().val, &mut second.borrow_mut().val);
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
        let debug_friendly_format: TreeRoot = input.into();
        assert_eq!(debug_friendly_format, expected);
    }

    #[test]
    fn case2() {
        let input: TreeRoot = "[3,1,4,null,null,2]".into();
        let mut input: Option<Rc<RefCell<TreeNode>>> = input.into();
        let expected: TreeRoot = "[2,1,4,null,null,3]".into();
        Solution::recover_tree(&mut input);
        let debug_friendly_format: TreeRoot = input.into();
        assert_eq!(debug_friendly_format, expected);
    }

    #[test]
    fn case3() {
        let input: TreeRoot = "[2,3,1]".into();
        let mut input: Option<Rc<RefCell<TreeNode>>> = input.into();
        let expected: TreeRoot = "[2,1,3]".into();
        Solution::recover_tree(&mut input);
        let debug_friendly_format: TreeRoot = input.into();
        assert_eq!(debug_friendly_format, expected);
    }

    #[test]
    fn case4() {
        let input: TreeRoot = "[2,3,1,null,null,null,5]".into();
        let mut input: Option<Rc<RefCell<TreeNode>>> = input.into();
        let expected: TreeRoot = "[2,1,3,null,null,null,5]".into();
        Solution::recover_tree(&mut input);
        let debug_friendly_format: TreeRoot = input.into();
        assert_eq!(debug_friendly_format, expected);
    }

    #[test]
    fn case5() {
        let input: TreeRoot = "[4,2,6,3]".into();
        let mut input: Option<Rc<RefCell<TreeNode>>> = input.into();
        let expected: TreeRoot = "[4,3,6,2]".into();
        Solution::recover_tree(&mut input);
        let debug_friendly_format: TreeRoot = input.into();
        assert_eq!(debug_friendly_format, expected);
    }
}
