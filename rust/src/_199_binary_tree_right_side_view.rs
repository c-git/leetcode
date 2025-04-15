//! Solution for https://leetcode.com/problems/binary-tree-right-side-view
//! 199. Binary Tree Right Side View

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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        let Some(root) = root else {
            return result;
        };
        let mut prev_level = -1;
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((root, 0));
        while let Some((node, level)) = queue.pop_front() {
            if level == prev_level {
                *result.last_mut().unwrap() = node.borrow().val;
            } else {
                prev_level = level;
                result.push(node.borrow().val);
            }
            if let Some(left) = node.borrow_mut().left.take() {
                queue.push_back((left, level + 1));
            }
            if let Some(right) = node.borrow_mut().right.take() {
                queue.push_back((right, level + 1));
            }
        }
        result
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
    #[case(TreeRoot::from("[1,2,3,null,5,null,4]").into(), vec![1,3,4])]
    #[case(TreeRoot::from("[1,2,3,4,null,null,null,5]").into(), vec![1,3,4,5])]
    #[case(TreeRoot::from("[1,null,3]").into(), vec![1,3])]
    #[case(TreeRoot::from("[]").into(), vec![])]
    fn case(#[case] root: Option<Rc<RefCell<TreeNode>>>, #[case] expected: Vec<i32>) {
        let actual = Solution::right_side_view(root);
        assert_eq!(actual, expected);
    }
}
