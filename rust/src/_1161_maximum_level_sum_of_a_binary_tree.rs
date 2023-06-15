//! Solution for https://leetcode.com/problems/maximum-level-sum-of-a-binary-tree
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
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = 0;
        let mut max_level_sum = i32::MIN;
        let mut queue = VecDeque::new();
        queue.push_back((1, root.unwrap()));
        let mut curr_level = 0;
        let mut curr_level_sum = max_level_sum;
        while let Some((level, node)) = queue.pop_front() {
            let node = node.borrow();
            if curr_level == level {
                // Increment current
                curr_level_sum += node.val;
            } else {
                // Start new level
                if curr_level_sum > max_level_sum {
                    max_level_sum = curr_level_sum;
                    result = curr_level;
                }
                curr_level = level;
                curr_level_sum = node.val;
            }
            if let Some(left) = node.left.as_ref().map(Rc::clone) {
                queue.push_back((level + 1, left));
            }
            if let Some(right) = node.right.as_ref().map(Rc::clone) {
                queue.push_back((level + 1, right));
            }
        }
        if curr_level_sum > max_level_sum {
            result = curr_level;
        }
        result
    }
}

struct Solution;
use cargo_leet::TreeNode;

#[cfg(test)]
mod tests {
    use super::*;
    use cargo_leet::TreeRoot;

    use rstest::rstest;

    #[rstest]
    #[case(TreeRoot::from("[1,7,0,7,-8,null,null]").into(), 2)]
    #[case(TreeRoot::from("[989,null,10250,98693,-89388,null,null,null,-32127]").into(), 2)]
    fn case(#[case] root: Option<Rc<RefCell<TreeNode>>>, #[case] expected: i32) {
        let actual = Solution::max_level_sum(root);
        assert_eq!(actual, expected);
    }
}
