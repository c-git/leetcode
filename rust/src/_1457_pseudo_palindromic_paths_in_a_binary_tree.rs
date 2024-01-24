//! Solution for https://leetcode.com/problems/pseudo-palindromic-paths-in-a-binary-tree
//! 1457. Pseudo-Palindromic Paths in a Binary Tree

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
    pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::pseudo_palindromic_paths_(&root, 0)
    }

    /// Checks how many paths from this node are pseudo-palindromic by using needing_match
    /// as a bitmap to track the numbers along the path that have not been matched yet
    /// If one or less numbers need to match then it's pseudo-palindromic because that one can go in the center
    fn pseudo_palindromic_paths_(
        root: &Option<Rc<RefCell<TreeNode>>>,
        mut needing_match: u16,
    ) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            let node_bit_value = 1 << node.val; // Create a number with one bit set that matches value
            needing_match ^= node_bit_value; // Toggle the bit for this node with xor

            let is_leaf = node.left.is_none() && node.right.is_none();
            if is_leaf {
                if needing_match.count_ones() <= 1 {
                    1 // This is pseudo-palindromic
                } else {
                    0 // This is not pseudo-palindromic
                }
            } else {
                Self::pseudo_palindromic_paths_(&node.left, needing_match)
                    + Self::pseudo_palindromic_paths_(&node.right, needing_match)
            }
        } else {
            0
        }
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
    #[case(TreeRoot::from("[2,3,1,3,1,null,1]").into(), 2)]
    #[case(TreeRoot::from("[2,1,1,1,3,null,null,null,null,null,1]").into(), 1)]
    #[case(TreeRoot::from("[9]").into(), 1)]
    fn case(#[case] root: Option<Rc<RefCell<TreeNode>>>, #[case] expected: i32) {
        let actual = Solution::pseudo_palindromic_paths(root);
        assert_eq!(actual, expected);
    }
}
