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
        // SEE PREVIOUS VERSION OF THIS FILE AS THIS IS NOT THE MOST EFFICIENT WAY TO SOLVE JUST MORE INTERESTING

        Self::height(root.clone());
        // Need to leak this memory because otherwise it will stack overflow trying to drop
        // NB: Could also have used forget just did two different ways to record both for future reference)
        let infected_root = Box::leak(Box::new(Self::make_infected_root(&root, start)));
        std::mem::forget(root); // Don't need it anymore but cannot let drop run otherwise we get stack overflow
        Self::height(infected_root.clone()) as i32 - 1
    }

    fn make_infected_root(root: &NodeOpt, val_infected: i32) -> NodeOpt {
        let mut stack = vec![(root.clone(), vec![])];
        while let Some((node, mut parents)) = stack.pop() {
            if let Some(node) = node {
                let node_inner = node.borrow();

                if node_inner.val == val_infected {
                    // Found the infected node this needs to be the new root
                    // To avoid complexity we are just going to take a copy of this node and
                    // a copy of its taller child as the left child and put
                    // the right child as it's parent with it's parents also inverted

                    let mut new_root = TreeNode::new(node_inner.val);

                    let left_height = Self::height(node_inner.left.clone());
                    let right_height = Self::height(node_inner.right.clone());

                    // Keep taller child as left
                    if left_height > right_height {
                        new_root.left = node_inner.left.clone();
                    } else {
                        new_root.left = node_inner.right.clone();
                    };

                    // Attach parents as right child
                    new_root.right = Self::parent_with_relevant_ancestors_as_children(
                        &parents[..],
                        new_root.val,
                    );

                    return Some(Rc::new(RefCell::new(new_root)));
                } else {
                    // keep searching for the infected node
                    parents.push(Rc::clone(&node));
                    stack.push((node_inner.left.clone(), parents.clone()));
                    stack.push((node_inner.right.clone(), parents.clone()));
                }
            }
        }
        unimplemented!("Should always find the infected node as it must exist by constraint");
    }

    fn height(root: NodeOpt) -> usize {
        let mut result = 0;
        let mut stack = vec![(root, 0)];
        while let Some((node, height)) = stack.pop() {
            if let Some(node) = node {
                let node = node.borrow();
                let new_height = height + 1;
                result = result.max(new_height);
                stack.push((node.left.clone(), new_height));
                stack.push((node.right.clone(), new_height));
            }
        }
        result
    }

    /// Must be called from a child and will keep the other child as the left child and it's parent if any as the right child
    fn parent_with_relevant_ancestors_as_children(
        parent_list: &[Rc<RefCell<TreeNode>>],
        child_to_drop_val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let node = if let Some(last) = parent_list.last() {
            Rc::clone(last)
        } else {
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
    #[ignore = "Still working on solution that is not recursive"]
    fn large_input() {
        let file_value = std::fs::read_to_string("large_inputs/2385_test_case 78.txt").unwrap();
        let root = TreeRoot::from(&file_value[..]);
        let start = 100000;
        let expected = 1; // Not known yet
        let actual = Solution::amount_of_time(root.into(), start);
        assert_eq!(actual, expected);
    }
}
