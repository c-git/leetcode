use std::cell::RefCell;
use std::rc::Rc;

use crate::helper::TreeNode;

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
#[derive(Eq, PartialEq)]
enum IterationState {
    BeforeFirst,
    AfterFirst,
}

impl IterationState {
    fn is_before_first(&self) -> bool {
        *self == Self::BeforeFirst
    }
}

struct BSTIterator {
    next_node: Option<Rc<RefCell<TreeNode>>>,
    stack: Vec<Rc<RefCell<TreeNode>>>,
    first_iteration: IterationState,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut result = Self {
            next_node: root,
            stack: vec![],
            first_iteration: IterationState::BeforeFirst,
        };
        result.move_to_next();
        result
    }

    fn move_to_next(&mut self) {
        // Assumption only called when either stack is not empty or currently on a node unless it's the first iteration

        if self.first_iteration.is_before_first() {
            self.first_iteration = IterationState::AfterFirst;
        } else {
            // Check right subtree
            self.next_node = self.next_node.clone().unwrap().borrow().right.clone();
        }

        // Go to leftmost node in this subtree
        while self.next_node.is_some() {
            self.stack.push(self.next_node.clone().unwrap());
            self.next_node = self.next_node.clone().unwrap().borrow().left.clone();
        }
        self.next_node = self.stack.pop();
    }

    fn next(&mut self) -> i32 {
        let result = self
            .next_node
            .clone()
            .expect("Question promises not to call after exhausted")
            .borrow()
            .val;

        self.move_to_next();
        result
    }

    fn has_next(&self) -> bool {
        self.next_node.is_some()
    }
}

#[cfg(test)]
mod tests {
    use crate::helper::TreeRoot;

    use super::*;

    #[test]
    fn case1() {
        let root: TreeRoot = "[7, 3, 15, null, null, 9, 20]".into();
        let mut bst_iter = BSTIterator::new(root.into());
        assert_eq!(bst_iter.next(), 3); // return 3
        assert_eq!(bst_iter.next(), 7); // return 7
        assert!(bst_iter.has_next()); // return True
        assert_eq!(bst_iter.next(), 9); // return 9
        assert!(bst_iter.has_next()); // return True
        assert_eq!(bst_iter.next(), 15); // return 15
        assert!(bst_iter.has_next()); // return True
        assert_eq!(bst_iter.next(), 20); // return 20
        assert!(!bst_iter.has_next()); // return False
    }
}
