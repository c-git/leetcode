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
use crate::helper::TreeNode;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // Clones are cheap because of Rc
        let mut result = vec![];

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
            result.push(current_node.clone().unwrap().borrow().val);

            // Check if this node has a right subtree
            current_node = current_node.unwrap().as_ref().borrow().right.clone();
        }

        result
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::helper::TreeRoot;

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
