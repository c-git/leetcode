//! Solution for https://leetcode.com/problems/lowest-common-ancestor-of-deepest-leaves
//! 1123. Lowest Common Ancestor of Deepest Leaves

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

struct NodeResult {
    lca: Rc<RefCell<TreeNode>>,
    deepest_leaf_height: usize,
}

impl Solution {
    pub fn lca_deepest_leaves(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Self::lca_deepest_leaves_(root.unwrap(), 0).lca)
    }

    fn lca_deepest_leaves_(node: Rc<RefCell<TreeNode>>, depth: usize) -> NodeResult {
        let left = node
            .borrow()
            .left
            .clone()
            .map(|x| Self::lca_deepest_leaves_(x, depth + 1));
        let right = node
            .borrow()
            .right
            .clone()
            .map(|x| Self::lca_deepest_leaves_(x, depth + 1));
        match (left, right) {
            (None, None) => NodeResult {
                lca: node,
                deepest_leaf_height: depth,
            },
            (None, Some(x)) | (Some(x), None) => x,
            (Some(left_result), Some(right_result)) => match left_result
                .deepest_leaf_height
                .cmp(&right_result.deepest_leaf_height)
            {
                std::cmp::Ordering::Less => right_result,
                std::cmp::Ordering::Equal => NodeResult {
                    lca: node,
                    deepest_leaf_height: left_result.deepest_leaf_height,
                },
                std::cmp::Ordering::Greater => left_result,
            },
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
    #[case(TreeRoot::from("[3,5,1,6,2,0,8,null,null,7,4]").into(), TreeRoot::from("[2,7,4]").into())]
    #[case(TreeRoot::from("[1]").into(), TreeRoot::from("[1]").into())]
    #[case(TreeRoot::from("[0,1,3,null,2]").into(), TreeRoot::from("[2]").into())]
    fn case(
        #[case] root: Option<Rc<RefCell<TreeNode>>>,
        #[case] expected: Option<Rc<RefCell<TreeNode>>>,
    ) {
        let actual = Solution::lca_deepest_leaves(root);
        assert_eq!(actual, expected);
    }
}
