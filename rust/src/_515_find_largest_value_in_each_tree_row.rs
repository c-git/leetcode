//! Solution for https://leetcode.com/problems/find-largest-value-in-each-tree-row
//! 515. Find Largest Value in Each Tree Row

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
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        if root.is_none() {
            return result;
        }

        let mut queue = VecDeque::new();
        queue.push_back((root.unwrap(), 0));
        let mut last_level = 0;
        let mut max = i32::MIN;

        while let Some((node, level)) = queue.pop_front() {
            if level == last_level {
                max = max.max(node.borrow().val);
            } else {
                result.push(max);
                last_level = level;
                max = node.borrow().val;
            }
            if let Some(left) = node.borrow_mut().left.take() {
                queue.push_back((left, level + 1));
            }
            if let Some(right) = node.borrow_mut().right.take() {
                queue.push_back((right, level + 1));
            }
        }
        result.push(max);

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
    #[case(TreeRoot::from("[1,3,2,5,3,null,9]").into(), vec![1,3,9])]
    #[case(TreeRoot::from("[1,2,3]").into(), vec![1,3])]
    fn case(#[case] root: Option<Rc<RefCell<TreeNode>>>, #[case] expected: Vec<i32>) {
        let actual = Solution::largest_values(root);
        assert_eq!(actual, expected);
    }
}
