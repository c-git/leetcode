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

// class Solution:
//     def build_bst(
//         self, min: int, max: int, preorder: list[int], index: int
//     ) -> tuple[TreeNode | None, int]:
//         if index == len(preorder):
//             return None, index
//         value = preorder[index]
//         if value < min or value > max:
//             return None, index
//         root = TreeNode(value)
//         index += 1
//         root.left, index = self.build_bst(min, value - 1, preorder, index)
//         root.right, index = self.build_bst(value + 1, max, preorder, index)
//         return root, index

//     def bstFromPreorder(self, preorder: list[int]) -> TreeNode | None:  # noqa: N802
//         return self.build_bst(0, 10**8 + 1, preorder, 0)[0]

impl Solution {
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        // Source: nasser python solution conversion
        Self::build_bst(0, 100_000_001, &preorder, 0).0
    }

    fn build_bst(
        min: i32,
        max: i32,
        preorder: &[i32],
        mut index: usize,
    ) -> (Option<Rc<RefCell<TreeNode>>>, usize) {
        if index == preorder.len() {
            return (None, index);
        }

        let value = preorder[index];
        if value < min || value > max {
            return (None, index);
        }
        let mut root = TreeNode::new(value);
        index += 1;
        (root.left, index) = Self::build_bst(min, value - 1, preorder, index);
        (root.right, index) = Self::build_bst(value + 1, max, preorder, index);
        (Some(Rc::new(RefCell::new(root))), index)
    }
}

use crate::helper::TreeNode;
struct Solution;
#[cfg(test)]
mod tests {
    use crate::helper::TreeRoot;

    use super::*;

    #[test]
    fn case1() {
        let input = vec![8, 5, 1, 7, 10, 12];
        let expected: TreeRoot = "[8, 5, 10, 1, 7, null, 12]".into();
        let actual = Solution::bst_from_preorder(input);
        assert_eq!(actual, expected.into());
    }

    #[test]
    fn case2() {
        let input = vec![1, 3];
        let expected: TreeRoot = "[1,null,3]".into();
        let actual = Solution::bst_from_preorder(input);
        assert_eq!(actual, expected.into());
    }
}
