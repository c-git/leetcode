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

use crate::helper::TreeNode;

type Node = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    fn build_tree_helper(inorder: &[i32], postorder: &[i32]) -> Node {
        debug_assert_eq!(inorder.len(), postorder.len());
        if postorder.is_empty() {
            return Node::None;
        }

        // Create root
        let root_val = postorder.last().expect("Already checked for empty slice");
        let mut root = TreeNode::new(*root_val);
        let root_index = inorder
            .iter()
            .position(|x| x == root_val)
            .expect("Constraints in question say that the values must exist in both");

        // Get left subtree
        let left_inorder = &inorder[..root_index];
        let left_postorder = &postorder[..left_inorder.len()];
        let left_node = Self::build_tree_helper(left_inorder, left_postorder);

        // Get right subtree
        let right_inorder = &inorder[root_index + 1..];
        let right_node = if right_inorder.is_empty() {
            Node::None
        } else {
            // If statement needed to prevent subtract with overflow
            let right_postorder = &postorder[left_inorder.len()..=postorder.len() - 2];
            Self::build_tree_helper(right_inorder, right_postorder)
        };

        // Attach children to the root
        root.left = left_node;
        root.right = right_node;

        Some(Rc::new(RefCell::new(root)))
    }

    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Node {
        debug_assert_eq!(inorder.len(), postorder.len());
        Self::build_tree_helper(&inorder, &postorder)
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use crate::helper::TreeRoot;

    use super::*;

    #[test]
    fn case1() {
        let inorder = vec![9, 3, 15, 20, 7];
        let postorder = vec![9, 15, 7, 20, 3];
        let expected: TreeRoot = "[3,9,20,null,null,15,7]".into();
        let actual = Solution::build_tree(inorder, postorder);
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
