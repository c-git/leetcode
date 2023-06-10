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

use cargo_leet::TreeNode;

type Node = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    fn build_tree_helper(preorder: &[i32], inorder: &[i32]) -> Node {
        debug_assert_eq!(preorder.len(), inorder.len());
        if preorder.is_empty() {
            return Node::None;
        }

        // Create root
        let root_val = preorder[0];
        let mut root = TreeNode::new(root_val);
        let root_index = inorder
            .iter()
            .position(|x| x == &root_val)
            .expect("Constraints in question say that the values must exist in both");

        // Get left subtree
        let left_inorder = &inorder[..root_index];
        let left_preorder = &preorder[1..=left_inorder.len()];
        let left_node = Self::build_tree_helper(left_preorder, left_inorder);

        // Get right subtree
        let right_inorder = &inorder[root_index + 1..];
        let right_preorder = &preorder[left_inorder.len() + 1..];
        let right_node = Self::build_tree_helper(right_preorder, right_inorder);

        // Attach children to the root
        root.left = left_node;
        root.right = right_node;

        Some(Rc::new(RefCell::new(root)))
    }

    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Node {
        debug_assert_eq!(preorder.len(), inorder.len());
        Self::build_tree_helper(&preorder, &inorder)
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use cargo_leet::TreeRoot;

    use super::*;

    #[test]
    fn case1() {
        let preorder = vec![3, 9, 20, 15, 7];
        let inorder = vec![9, 3, 15, 20, 7];
        let expected: TreeRoot = "[3,9,20,null,null,15,7]".into();
        let actual = Solution::build_tree(preorder, inorder);
        assert_eq!(actual, expected.into());
    }

    #[test]
    fn case2() {
        let preorder = vec![-1];
        let inorder = vec![-1];
        let expected: TreeRoot = "[-1]".into();
        let actual = Solution::build_tree(preorder, inorder);
        assert_eq!(actual, expected.into());
    }
}
