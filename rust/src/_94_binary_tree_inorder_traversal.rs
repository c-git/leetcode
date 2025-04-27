use std::cell::RefCell;
use std::rc::Rc;

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

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // Clones are cheap because of Rc
        let mut result = vec![];

        let mut current_node = root;
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![];
        while !stack.is_empty() || current_node.is_some() {
            // Go to leftmost node
            while let Some(node) = current_node.as_ref() {
                stack.push(Rc::clone(node));
                let temp = node.borrow().left.clone();
                current_node = temp;
            }
            // Recover leftmost node from stack
            let leftmost_node = stack
                .pop()
                .expect("stack cannot be empty because to enter the loop either the stack has to be non-empty or current_node must be some. If stack was empty then the current_node at a minium would be added to the stack");

            // Perform action
            result.push(leftmost_node.borrow().val);

            // Check if this node has a right subtree
            current_node.clone_from(&leftmost_node.borrow().right);
        }

        result
    }
}

use cargo_leet::TreeNode;
pub struct Solution;

#[cfg(test)]
mod tests {
    use cargo_leet::TreeRoot;

    use super::*;

    #[test]
    fn case1() {
        let input: TreeRoot = "[1,null,2,3]".into();
        let expected = vec![1, 3, 2];
        let actual = Solution::inorder_traversal(input.into());
        assert_eq!(actual, expected);
    }

    #[test]
    fn case2() {
        let input: TreeRoot = "[]".into();
        let expected: Vec<i32> = vec![];
        let actual = Solution::inorder_traversal(input.into());
        assert_eq!(actual, expected);
    }

    #[test]
    fn case3() {
        let input: TreeRoot = "[1]".into();
        let expected = vec![1];
        let actual = Solution::inorder_traversal(input.into());
        assert_eq!(actual, expected);
    }
}
