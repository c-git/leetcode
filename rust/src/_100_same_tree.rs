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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        p == q
    }
}

struct Solution;
use crate::helper::TreeNode;
#[cfg(test)]
mod tests {
    use crate::helper::TreeRoot;

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("[1,2,3]", "[1,2,3]", true)]
    #[case("[1,2]", "[1,null,2]", false)]
    #[case("[1,2,1]", "[1,1,2]", false)]
    fn case(#[case] p: &str, #[case] q: &str, #[case] expected: bool) {
        let p: TreeRoot = p.into();
        let q: TreeRoot = q.into();
        let actual = Solution::is_same_tree(p.into(), q.into());
        assert_eq!(actual, expected);
    }
}
