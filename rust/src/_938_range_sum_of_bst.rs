//! Solution for https://leetcode.com/problems/range-sum-of-bst
//! 938. Range Sum of BST

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
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        match root {
            Some(root) => {
                let ref_cell = Rc::into_inner(root)
                    .expect("We should only have one as we started from the root");
                let node = ref_cell.into_inner();
                let TreeNode { val, left, right } = node;
                (if val >= low && val <= high { val } else { 0 })
                    + if val >= low {
                        Self::range_sum_bst(left, low, high)
                    } else {
                        0
                    }
                    + if val <= high {
                        Self::range_sum_bst(right, low, high)
                    } else {
                        0
                    }
            }
            None => 0,
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
    #[case(TreeRoot::from("[10,5,15,3,7,null,18]").into(), 7, 15, 32)]
    #[case(TreeRoot::from("[10,5,15,3,7,13,18,1,null,6]").into(), 6, 10, 23)]
    fn case(
        #[case] root: Option<Rc<RefCell<TreeNode>>>,
        #[case] low: i32,
        #[case] high: i32,
        #[case] expected: i32,
    ) {
        let actual = Solution::range_sum_bst(root, low, high);
        assert_eq!(actual, expected);
    }
}
