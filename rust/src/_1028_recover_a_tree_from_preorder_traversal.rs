//! Solution for https://leetcode.com/problems/recover-a-tree-from-preorder-traversal
//! 1028. Recover a Tree From Preorder Traversal

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
type Depth = usize;
type Position = usize;
enum State {
    DepthCheck(Depth),
    FindEndNumberRange(Depth, Position),
}

impl Solution {
    /// Solves the problem by breaking it up into smaller problems.
    /// - First the input is converted from a string into a vector of the depths
    ///   and related nodes
    /// - Then we prepare to iterate through the vector by creating the root
    /// - We then put the root onto a stack which will hold the list of nodes we
    ///   "may" add children to
    /// - Then we loop through the list of nodes we got from the input and if
    ///   the depth means it's a child of the node at the top of the stack we
    ///   add it as the left child if it is not we keep going up until we find
    ///   it's parent and set it as the right child then loop again
    pub fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
        if traversal.is_empty() {
            return None;
        }
        let node_list = simplify_input(traversal);

        let root = Some(Rc::new(RefCell::new(TreeNode::new(node_list[0].1.val))));
        let mut node_stack = vec![(0, Rc::clone(root.as_ref().unwrap()))];
        for (next_depth, next_node) in node_list.into_iter().skip(1) {
            let next_node = Rc::new(RefCell::new(next_node));
            if next_depth == node_stack.last().unwrap().0 + 1 {
                // Is left child of the top most node on the stack
                node_stack.last().unwrap().1.borrow_mut().left = Some(Rc::clone(&next_node));
                node_stack.push((next_depth, next_node));
            } else {
                // Not child of topmost node so must be right child of another
                // node in the stack
                loop {
                    let (top_depth, top_node) = node_stack.pop().unwrap();
                    if next_depth == top_depth + 1 {
                        debug_assert!(
                            top_node.borrow().left.is_some(),
                            "Left child should already have been set"
                        );
                        top_node.borrow_mut().right = Some(Rc::clone(&next_node));
                        node_stack.push((next_depth, next_node));
                        break;
                    }
                }
            }
        }

        root
    }
}

/// Converts the string provided into a vector of the information in the string
fn simplify_input(traversal: String) -> Vec<(Depth, TreeNode)> {
    let mut result: Vec<(usize, TreeNode)> = vec![];
    let mut state = State::DepthCheck(0);
    for (i, c) in traversal.as_bytes().iter().enumerate() {
        state = match state {
            State::DepthCheck(depth) => match c {
                b'-' => State::DepthCheck(depth + 1),
                b'0'..=b'9' => State::FindEndNumberRange(depth, i),
                _ => unreachable!("Problem guarantees only numbers and dashes"),
            },
            State::FindEndNumberRange(depth, start_pos) => match c {
                b'-' => {
                    result.push((
                        depth,
                        TreeNode::new(traversal[start_pos..i].parse().unwrap()),
                    ));
                    State::DepthCheck(1)
                }
                b'0'..=b'9' => state, // No change number continues
                _ => unreachable!("Problem guarantees only numbers and dashes"),
            },
        }
    }
    if let State::FindEndNumberRange(depth, start_pos) = state {
        // Add last node
        result.push((
            depth,
            TreeNode::new(traversal[start_pos..].parse().unwrap()),
        ));
    } else {
        unreachable!("String must end with a node value")
    };
    debug_assert_eq!(result[0].0, 0, "First node must be the root");
    result
}

// << ---------------- Code below here is only for local use ---------------- >>

use cargo_leet::TreeNode;
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use cargo_leet::TreeRoot;
    use rstest::rstest;

    #[rstest]
    #[case("1-2--3--4-5--6--7", TreeRoot::from("[1,2,5,3,4,6,7]").into())]
    #[case("1-2--3---4-5--6---7", TreeRoot::from("[1,2,5,3,null,6,null,4,null,7]").into())]
    #[case("1-401--349---90--88", TreeRoot::from("[1,401,null,349,88,90]").into())]
    fn case(#[case] traversal: String, #[case] expected: Option<Rc<RefCell<TreeNode>>>) {
        let actual = Solution::recover_from_preorder(traversal);
        assert_eq!(actual, expected);
    }
}
