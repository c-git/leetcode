//! Solution for https://leetcode.com/problems/amount-of-time-for-binary-tree-to-be-infected
//! 2385. Amount of Time for Binary Tree to Be Infected

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: NodeOpt,
//   pub right: NodeOpt,
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
type NodeOpt = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn amount_of_time(root: NodeOpt, start: i32) -> i32 {
        let root = Self::make_infected_root(&root, start, vec![]).expect("Must exist");
        let node = root.borrow();
        Self::height(&node.left).max(Self::height(&node.right)) as _
    }

    fn make_infected_root<'a>(
        root: &'a NodeOpt,
        val_infected: i32,
        mut parent_list: Vec<&'a NodeOpt>,
    ) -> NodeOpt {
        // Bug in clippy https://github.com/rust-lang/rust-clippy/issues/8281#issuecomment-1703932391
        #[allow(clippy::question_mark)]
        let Some(node) = root
        else {
            return None;
        };
        let node = node.borrow();
        if node.val == val_infected {
            // Found the infected node this needs to be the new root
            // To avoid complexity we are just going to take a copy of this node and it shortest child the other child gets replaced with the parent of this node
            let left_height = Self::height(&node.left);
            let right_height = Self::height(&node.right);

            // For simplicity ensure to keep longer child on left in copy
            let child = if left_height > right_height {
                &node.left
            } else {
                &node.right
            };
            let mut new_root = TreeNode::new(node.val);
            new_root.left = child.clone();

            // Attach parents as right child
            new_root.right =
                Self::parent_with_relevant_ancestors_as_children(&parent_list[..], new_root.val);

            Some(Rc::new(RefCell::new(new_root)))
        } else {
            // keep searching for the infected node
            parent_list.push(root);
            let left = Self::make_infected_root(&node.left, val_infected, parent_list.clone());
            if left.is_some() {
                return left;
            }
            Self::make_infected_root(&node.right, val_infected, parent_list)
        }
    }

    fn height(root: &NodeOpt) -> usize {
        if let Some(root) = root {
            let node = root.borrow();
            Self::height(&node.left).max(Self::height(&node.right)) + 1
        } else {
            0
        }
    }

    /// Must be called from a child and will keep the other child as the left child and it's parent if any as the right child
    fn parent_with_relevant_ancestors_as_children(
        parent_list: &[&Option<Rc<RefCell<TreeNode>>>],
        child_to_drop_val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let node = *(parent_list.last()?);
        let Some(node) = node else {
            return None;
        };
        let original_node = node.borrow();
        let mut result = TreeNode::new(original_node.val);

        // Determine which child to keep
        match (&original_node.left, &original_node.right) {
            (None, None) => unreachable!(
                "How did we get here without a child. A child is supposed to call this function"
            ),
            (None, Some(child)) | (Some(child), None) => {
                // We only have one child so we'll have no left child and put our parent on the right
                debug_assert_eq!(
                    child.borrow().val,
                    child_to_drop_val,
                    "Only one child must be the child that called this function"
                )
            }
            (Some(left), Some(right)) => {
                if left.borrow().val == child_to_drop_val {
                    result.left = Some(Rc::clone(right));
                } else {
                    debug_assert_eq!(
                        right.borrow().val,
                        child_to_drop_val,
                        "Either left or right must be the calling child"
                    );
                    result.left = Some(Rc::clone(left));
                }
            }
        }

        // Attach parent
        result.right = Self::parent_with_relevant_ancestors_as_children(
            &parent_list[..parent_list.len() - 1],
            result.val,
        );

        Some(Rc::new(RefCell::new(result)))
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
    #[case(TreeRoot::from("[1,5,3,null,4,10,6,9,2]").into(), 3, 4)]
    #[case(TreeRoot::from("[1]").into(), 1, 0)]
    #[case(TreeRoot::from("[1,2,null,3,null,4,null,5]").into(), 1, 4)]
    #[case(TreeRoot::from("[1,2,null,3,null,4,null,5]").into(), 2, 3)]
    #[case(TreeRoot::from("[1,2,null,3,null,4,null,5]").into(), 4, 3)]
    #[case(TreeRoot::from("[1,2,null,3,null,4,null,5]").into(), 4, 3)]
    #[case(TreeRoot::from("[5,2,3,4,null,null,null,1]").into(), 4, 3)]
    fn case(#[case] root: NodeOpt, #[case] start: i32, #[case] expected: i32) {
        let actual = Solution::amount_of_time(root, start);
        assert_eq!(actual, expected);
    }

    #[test]
    fn large_input() {
        let file_value = std::fs::read_to_string("large_inputs/2385_test_case 78.txt").unwrap();
        let root = TreeRoot::from(&file_value[..]);
        let start = 100000;
        let expected = 1; // Not known yet
        let actual = Solution::amount_of_time(root.into(), start);
        assert_eq!(actual, expected);
    }
}
