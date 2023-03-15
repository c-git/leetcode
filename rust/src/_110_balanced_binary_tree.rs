use std::cell::RefCell;
use std::cmp::max;
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
use std::rc::Rc;

use crate::helper::TreeNode;

struct Solution;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self.not_bal_or_max_height(root).is_balanced()
    }
    fn not_bal_or_max_height(&self, root: Option<Rc<RefCell<TreeNode>>>) -> MaxBalancedHeight {
        if let Some(root) = root {
            let left_height = Self.not_bal_or_max_height(root.borrow().left.clone());
            if !left_height.is_balanced() {
                return MaxBalancedHeight::NotBalanced;
            }
            let right_height = Self.not_bal_or_max_height(root.borrow().right.clone());
            if !right_height.is_balanced() {
                return MaxBalancedHeight::NotBalanced;
            }
            match (left_height, right_height) {
                (MaxBalancedHeight::Balanced(left), MaxBalancedHeight::Balanced(right)) => {
                    // let diff = left.abs_diff(right); // Rust version LeetCode too old to use this
                    let diff = if left > right {
                        left - right
                    } else {
                        right - left
                    };
                    if diff > 1 {
                        MaxBalancedHeight::NotBalanced
                    } else {
                        MaxBalancedHeight::Balanced(1 + max(left, right))
                    }
                }
                _ => unreachable!("Both should be balanced as per checks above"),
            }
        } else {
            // Empty trees are considered balanced and of height 0
            MaxBalancedHeight::Balanced(0)
        }
    }
}

enum MaxBalancedHeight {
    Balanced(u32),
    NotBalanced,
}

impl MaxBalancedHeight {
    #[must_use]
    fn is_balanced(&self) -> bool {
        matches!(self, Self::Balanced(_))
    }
}

#[cfg(test)]
mod tests {
    use crate::helper::TreeRoot;

    use super::*;

    #[test]
    fn case1() {
        let input: TreeRoot = "[3,9,20,null,null,15,7]".into();
        let expected = true;

        let actual = Solution::is_balanced(input.into());
        assert_eq!(actual, expected);
    }

    #[test]
    fn case2() {
        let input: TreeRoot = "[1,2,2,3,3,null,null,4,4]".into();
        let expected = false;

        let actual = Solution::is_balanced(input.into());
        assert_eq!(actual, expected);
    }

    #[test]
    fn case3() {
        let input: TreeRoot = "[]".into();
        let expected = true;

        let actual = Solution::is_balanced(input.into());
        assert_eq!(actual, expected);
    }
}
