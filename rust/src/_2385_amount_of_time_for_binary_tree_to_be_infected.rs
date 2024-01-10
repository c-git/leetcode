//! Solution for https://leetcode.com/problems/amount-of-time-for-binary-tree-to-be-infected
//! 2385. Amount of Time for Binary Tree to Be Infected

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
struct ResultInfo {
    height: usize,
    start_height_from_top: Option<usize>,
    time: Option<usize>,
}
impl Solution {
    pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
        Self::check_node(&root, start)
            .time
            .expect("Constraints say start must exist so this must also exist") as i32
    }

    fn check_node(root: &Option<Rc<RefCell<TreeNode>>>, start: i32) -> ResultInfo {
        if let Some(node) = root {
            let node = node.as_ref().borrow();
            let left = Self::check_node(&node.left, start);
            let right = Self::check_node(&node.right, start);
            let height = left.height.max(right.height) + 1;
            let start_height_from_top = match (
                left.start_height_from_top,
                right.start_height_from_top,
                node.val == start,
            ) {
                (None, None, true) => Some(0),
                (None, None, false) => None,
                (Some(_), Some(_), false) | (_, _, true) => {
                    unreachable!("All values in true should be unique")
                }
                (Some(x), None, false) | (None, Some(x), false) => Some(x + 1),
            };

            let time = if start_height_from_top.is_some() {
                match (left.time, right.time) {
                (None, None) => {
                    debug_assert!(node.val == start, "This case should only happen when we just found the start value");
                    Some(height-1) // Minus 1 because this node starts off infected
                }, 
                (None, Some(x)) => Some(x.max(left.height)+1),
                (Some(x), None) => Some(x.max(right.height)+1),
                (Some(_), Some(_)) => unreachable!("Both sides should not have a time because time is only present when start is found"),
            }
            } else {
                None
            };
            ResultInfo {
                height,
                start_height_from_top,
                time,
            }
        } else {
            ResultInfo {
                height: 0,
                start_height_from_top: None,
                time: None,
            }
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
    #[case(TreeRoot::from("[1,5,3,null,4,10,6,9,2]").into(), 3, 4)]
    #[case(TreeRoot::from("[1]").into(), 1, 0)]
    #[case(TreeRoot::from("[1,2,null,3,null,4,null,5]").into(), 1, 4)]
    fn case(
        #[case] root: Option<Rc<RefCell<TreeNode>>>,
        #[case] start: i32,
        #[case] expected: i32,
    ) {
        let actual = Solution::amount_of_time(root, start);
        assert_eq!(actual, expected);
    }
}
