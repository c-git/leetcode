use std::cell::RefCell;
use std::rc::Rc;

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
use crate::helper::TreeNode;

enum SearchResult {
    SwappedNode(Rc<RefCell<TreeNode>>),
    NotFound,
    Completed,
}

impl Solution {
    fn recover_tree_helper(
        root: Option<Rc<RefCell<TreeNode>>>,
        min: Option<i32>,
        max: Option<i32>,
    ) -> SearchResult {
        let root = if let Some(val) = root {
            val
        } else {
            return SearchResult::NotFound;
        };

        let root_val = root.borrow().val;

        if let Some(min) = min {
            if root_val <= min {
                return SearchResult::SwappedNode(root.clone());
            }
        }
        if let Some(max) = max {
            if root_val >= max {
                return SearchResult::SwappedNode(root.clone());
            }
        }

        let left = root.borrow().left.clone();
        match Self::recover_tree_helper(left, min, Some(root_val)) {
            SearchResult::SwappedNode(swapped_node) => {
                return if swapped_node.borrow().val > root_val {
                    root.borrow_mut().val = swapped_node.borrow().val;
                    swapped_node.borrow_mut().val = root_val;
                    SearchResult::Completed
                } else {
                    SearchResult::SwappedNode(swapped_node) // Swap needs to happen higher up
                };
            }
            SearchResult::NotFound => { /* Do nothing check other child */ }
            SearchResult::Completed => return SearchResult::Completed,
        }

        let right = root.borrow().right.clone();
        let result = match Self::recover_tree_helper(right, Some(root_val), max) {
            SearchResult::SwappedNode(swapped_node) => {
                if swapped_node.borrow().val < root_val {
                    root.borrow_mut().val = swapped_node.borrow().val;
                    swapped_node.borrow_mut().val = root_val;
                    SearchResult::Completed
                } else {
                    SearchResult::SwappedNode(swapped_node) // Swap needs to happen higher up
                }
            }
            SearchResult::NotFound => SearchResult::NotFound,
            SearchResult::Completed => SearchResult::Completed,
        };
        result
    }

    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        Self::recover_tree_helper(root.clone(), None, None);
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::helper::TreeRoot;

    use super::*;

    #[test]
    fn case1() {
        let input: TreeRoot = "[1,3,null,null,2]".into();
        let mut input: Option<Rc<RefCell<TreeNode>>> = input.into();
        let expected: TreeRoot = "[3,1,null,null,2]".into();
        Solution::recover_tree(&mut input);
        assert_eq!(expected, input.into());
    }

    #[test]
    fn case2() {
        let input: TreeRoot = "[3,1,4,null,null,2]".into();
        let mut input: Option<Rc<RefCell<TreeNode>>> = input.into();
        let expected: TreeRoot = "[2,1,4,null,null,3]".into();
        Solution::recover_tree(&mut input);
        assert_eq!(expected, input.into());
    }
}
