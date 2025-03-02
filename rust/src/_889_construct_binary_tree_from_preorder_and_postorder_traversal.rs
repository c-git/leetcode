//! Solution for https://leetcode.com/problems/construct-binary-tree-from-preorder-and-postorder-traversal
//! 889. Construct Binary Tree from Preorder and Postorder Traversal

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
use std::collections::BTreeSet;
use std::rc::Rc;

impl Solution {
    /// Preorder tells us the next node to add and post order tells us the next
    /// leaf (or if seen, node that is completed)
    pub fn construct_from_pre_post(
        preorder: Vec<i32>,
        postorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() {
            return None;
        }

        // Should never get all the way to the end so we can just unwrap calls to next
        let mut leaf_iter = postorder.into_iter();
        let mut next_leaf = leaf_iter.next().unwrap();
        let mut seen_values = BTreeSet::new();
        seen_values.insert(preorder[0]);
        // Stack never expected to be empty as leaves get added
        let mut stack = vec![Rc::new(RefCell::new(TreeNode::new(preorder[0])))];
        let root = Some(Rc::clone(stack.first().unwrap()));
        if preorder.len() == 1 {
            return root;
        }
        for next_value in preorder.into_iter().skip(1) {
            seen_values.insert(next_value);
            let next_node = Rc::new(RefCell::new(TreeNode::new(next_value)));
            let parent = stack.last().unwrap();
            if parent.borrow().left.is_some() {
                // Add as right child
                debug_assert!(parent.borrow().right.is_none());
                parent.borrow_mut().right = Some(Rc::clone(&next_node));
            } else {
                // Add as left child
                parent.borrow_mut().left = Some(Rc::clone(&next_node));
            }
            if next_leaf == next_value {
                // This is a leaf do not add to the and find next leaf
                loop {
                    next_leaf = match leaf_iter.next() {
                        Some(x) => x,
                        None => return root,
                    };
                    if seen_values.contains(&next_leaf) {
                        // This is a previous node remove from the stack as it
                        // cannot have more children
                        loop {
                            let top_node = stack.pop().unwrap();
                            if top_node.borrow().val == next_leaf {
                                // Found the one to remove we can stop now
                                break;
                            }
                        }
                    } else {
                        // New leaf found
                        break;
                    }
                }
            } else {
                stack.push(next_node);
            }
        }
        unreachable!("Should match with postorder and exit via no more values in post order")
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;
use cargo_leet::TreeNode;

#[cfg(test)]
mod tests {
    use super::*;

    use cargo_leet::TreeRoot;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,4,5,3,6,7], vec![4,5,2,6,7,3,1], TreeRoot::from("[1,2,3,4,5,6,7]").into())]
    #[case(vec![1], vec![1], TreeRoot::from("[1]").into())]
    #[case(vec![1,2,4,3,6,5], vec![4,2,6,5,3,1], TreeRoot::from("[1,2,3,4,null,6,5]").into())]
    fn case(
        #[case] preorder: Vec<i32>,
        #[case] postorder: Vec<i32>,
        #[case] expected: Option<Rc<RefCell<TreeNode>>>,
    ) {
        let actual = Solution::construct_from_pre_post(preorder, postorder);
        assert_eq!(actual, expected);
    }
}
