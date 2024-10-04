//! Solution for https://leetcode.com/problems/balanced-binary-tree
//! 110. Balanced Binary Tree

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

enum BalanceState {
    Balanced { height: usize },
    NotBalanced,
}

impl BalanceState {
    /// Returns `true` if the balance state is [`IsBalanced`].
    ///
    /// [`IsBalanced`]: BalanceState::IsBalanced
    #[must_use]
    fn is_balanced(&self) -> bool {
        matches!(self, Self::Balanced { .. })
    }
}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_balance_and_height(&root).is_balanced()
    }

    fn is_balance_and_height(root: &Option<Rc<RefCell<TreeNode>>>) -> BalanceState {
        let Some(root) = root else {
            return BalanceState::Balanced { height: 0 };
        };
        let root = root.borrow(); // Get borrowed version
        match (
            Self::is_balance_and_height(&root.left),
            Self::is_balance_and_height(&root.right),
        ) {
            (
                BalanceState::Balanced {
                    height: left_height,
                },
                BalanceState::Balanced {
                    height: right_height,
                },
            ) => {
                if left_height.abs_diff(right_height) > 1 {
                    BalanceState::NotBalanced
                } else {
                    BalanceState::Balanced {
                        height: 1 + left_height.max(right_height),
                    }
                }
            }
            _ => BalanceState::NotBalanced,
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
    #[case(TreeRoot::from("[3,9,20,null,null,15,7]").into(), true)]
    #[case(TreeRoot::from("[1,2,2,3,3,null,null,4,4]").into(), false)]
    #[case(TreeRoot::from("[]").into(), true)]
    fn case(#[case] root: Option<Rc<RefCell<TreeNode>>>, #[case] expected: bool) {
        let actual = Solution::is_balanced(root);
        assert_eq!(actual, expected);
    }
}
