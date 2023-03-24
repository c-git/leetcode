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

#[derive(Eq, PartialEq, Debug)]
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

        fn swap(a: Rc<RefCell<TreeNode>>, b: Rc<RefCell<TreeNode>>) {
            let temp = a.borrow().val;
            a.borrow_mut().val = b.borrow().val;
            b.borrow_mut().val = temp;
        }

        let left = root.borrow().left.clone();
        let left_result = match Self::recover_tree_helper(left, min, Some(root_val)) {
            SearchResult::SwappedNode(swapped_node) => {
                if swapped_node.borrow().val > root_val {
                    SearchResult::SwappedNode(swapped_node)
                } else {
                    return SearchResult::SwappedNode(swapped_node); // Swap needs to happen higher up
                }
            }
            SearchResult::NotFound => SearchResult::NotFound,
            SearchResult::Completed => return SearchResult::Completed,
        };

        let right = root.borrow().right.clone();
        let right_result = match Self::recover_tree_helper(right, Some(root_val), max) {
            SearchResult::SwappedNode(swapped_node) => {
                if swapped_node.borrow().val < root_val {
                    SearchResult::SwappedNode(swapped_node)
                } else {
                    assert_eq!(left_result, SearchResult::NotFound); // Based on question stating there can only be 1
                    return SearchResult::SwappedNode(swapped_node); // Swap needs to happen higher up
                }
            }
            SearchResult::NotFound => SearchResult::NotFound,
            SearchResult::Completed => return SearchResult::Completed,
        };

        match (left_result, right_result) {
            (SearchResult::SwappedNode(left_node), SearchResult::SwappedNode(right_node)) => {
                // Swap the children nodes
                swap(left_node, right_node);
                SearchResult::Completed
            }
            (SearchResult::SwappedNode(left_node), SearchResult::NotFound) => {
                swap(left_node, root);
                SearchResult::Completed
            }
            (SearchResult::NotFound, SearchResult::SwappedNode(right_node)) => {
                swap(right_node, root);
                SearchResult::Completed
            }
            (SearchResult::NotFound, SearchResult::NotFound) => SearchResult::NotFound,
            (_, _) => unreachable!("All SearchResult::Completed should already have been returned"),
        }
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

    #[test]
    fn case3() {
        let input: TreeRoot = "[2,3,1]".into();
        let mut input: Option<Rc<RefCell<TreeNode>>> = input.into();
        let expected: TreeRoot = "[2,1,3]".into();
        Solution::recover_tree(&mut input);
        assert_eq!(expected, input.into());
    }
}
