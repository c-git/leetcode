//! Solution for https://leetcode.com/problems/leaf-similar-trees
//! 872. Leaf-Similar Trees

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
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut leaves1 = vec![];
        let mut leaves2 = vec![];
        Self::leaves(&root1, &mut leaves1);
        Self::leaves(&root2, &mut leaves2);

        leaves1 == leaves2
    }

    fn leaves(root: &Option<Rc<RefCell<TreeNode>>>, leaves: &mut Vec<i32>) {
        if let Some(node) = root {
            let node = node.borrow();
            let is_leaf = node.left.is_none() && node.right.is_none();
            if is_leaf {
                leaves.push(node.val);
            } else {
                Self::leaves(&node.left, leaves);
                Self::leaves(&node.right, leaves);
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
    #[case(TreeRoot::from("[3,5,1,6,2,9,8,null,null,7,4]").into(), TreeRoot::from("[3,5,1,6,7,4,2,null,null,null,null,null,null,9,8]").into(), true)]
    #[case(TreeRoot::from("[1,2,3]").into(), TreeRoot::from("[1,3,2]").into(), false)]
    fn case(
        #[case] root1: Option<Rc<RefCell<TreeNode>>>,
        #[case] root2: Option<Rc<RefCell<TreeNode>>>,
        #[case] expected: bool,
    ) {
        let actual = Solution::leaf_similar(root1, root2);
        assert_eq!(actual, expected);
    }
}
