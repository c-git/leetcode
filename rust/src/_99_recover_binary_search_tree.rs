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
        // Source: Sample 0ms solution after completing problem

        let mut first = Node::None;
        let mut second = Node::None;
        let mut pre = Node::None;

        let mut root = root.as_ref().map(Rc::clone);
        let mut stack = std::collections::LinkedList::new();
        while root.is_some() || !stack.is_empty() {
            if let Some(node) = root {
                stack.push_front(Rc::clone(&node));
                root = node.borrow().left.as_ref().map(Rc::clone);
            } else if let Some(ref node) = stack.pop_front() {
                let node_val = node.borrow().val;

                if let Some(ref pre_node) = pre {
                    let pre_val = pre_node.borrow().val;
                    if pre_val > node_val {
                        if first.is_none() {
                            first = Some(Rc::clone(pre_node));
                        }
                        second = Some(Rc::clone(node));
                    } else {
                        pre = Some(Rc::clone(node));
                    }
                } else {
                    pre = Some(Rc::clone(node));
                }

                root = node.borrow().right.as_ref().map(Rc::clone);
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
