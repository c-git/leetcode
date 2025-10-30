//! Solution for https://leetcode.com/problems/serialize-and-deserialize-binary-tree
//! 297. Serialize and Deserialize Binary Tree

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
pub struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    #[expect(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }

    #[expect(clippy::only_used_in_recursion)]
    pub fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        match root {
            Some(root) => {
                let mut root = root.borrow_mut();
                let value = root.val;
                let left = self.serialize(root.left.take());
                let right = self.serialize(root.right.take());
                format!("({value},{left},{right})")
            }
            None => "()".to_string(),
        }
    }

    pub fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        Self::deserialize_(&data)
    }

    pub fn deserialize_(data: &str) -> Option<Rc<RefCell<TreeNode>>> {
        debug_assert_eq!(data.chars().next(), Some('('));
        debug_assert_eq!(data.chars().last(), Some(')'));

        // Pop off brackets
        let data = &data[1..data.len() - 1];

        if data.is_empty() {
            return None;
        }

        // Find value
        let first_comma_idx = data.find(',').unwrap();
        let val: i32 = data[..first_comma_idx].parse().unwrap();

        // Get left
        debug_assert_eq!(&data[first_comma_idx..first_comma_idx + 2], ",(");
        let mut second_comma_idx = first_comma_idx + 2; // Starts after the first open bracket
        let mut bracket_count = 1;
        while bracket_count > 0 {
            match &data[second_comma_idx..second_comma_idx + 1] {
                "(" => bracket_count += 1,
                ")" => bracket_count -= 1,
                _ => {}
            }
            second_comma_idx += 1;
        }
        let left = Self::deserialize_(&data[first_comma_idx + 1..second_comma_idx]);

        // Get right
        let right = Self::deserialize_(&data[second_comma_idx + 1..]);

        // Construct and return result
        let mut result = TreeNode::new(val);
        result.left = left;
        result.right = right;
        Some(Rc::new(RefCell::new(result)))
    }
}

// << ---------------- Code below here is only for local use ---------------- >>
use cargo_leet::TreeNode;

#[cfg(test)]
mod tests {
    use super::*;
    use cargo_leet::TreeRoot;
    use rstest::rstest;

    #[rstest]
    #[case("[1,2,3,null,null,4,5]")]
    #[case("[]")]
    #[case("[1,2,3,null,null,-584,5]")]
    #[case("[1,2,1000,null,null,-999,-1000]")]
    fn case(#[case] tree_as_str: &str) {
        let codec = Codec::new();
        let root: Option<Rc<RefCell<TreeNode>>> = TreeRoot::from(tree_as_str).into();
        let expected: Option<Rc<RefCell<TreeNode>>> = TreeRoot::from(tree_as_str).into();
        let str_repr = codec.serialize(root);
        let actual = codec.deserialize(str_repr);
        assert_eq!(actual, expected);
    }
}
