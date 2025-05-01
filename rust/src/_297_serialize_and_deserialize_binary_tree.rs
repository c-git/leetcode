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
struct Codec;

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let Some(root) = root else {
            return "()".into();
        };
        let left = root.borrow_mut().left.clone();
        let right = root.borrow_mut().right.clone();
        format!(
            "({},{},{})",
            root.borrow().val,
            Self.serialize(left),
            Self.serialize(right)
        )
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        Self::deserialize_(data.as_str())
    }
    fn deserialize_(data: &str) -> Option<Rc<RefCell<TreeNode>>> {
        assert_eq!(data.as_bytes()[0], b'(');
        assert_eq!(data.as_bytes().last(), Some(&b')'));
        assert!(data.len() >= 2);
        if data.len() == 2 {
            return None;
        }

        let comma_idx = data
            .char_indices()
            .find_map(|(idx, c)| if c == ',' { Some(idx) } else { None })
            .unwrap();
        let val = &data[1..comma_idx];
        let val: i32 = val.parse().unwrap();
        let mut open_count: u16 = 0;
        let second_comma = data
            .char_indices()
            .skip(comma_idx + 1)
            .find_map(|(idx, c)| {
                // Find a comma that is not nested
                match c {
                    '(' => open_count += 1,
                    ')' => open_count -= 1,
                    ',' if open_count == 0 => return Some(idx),
                    _ => {}
                }
                None
            })
            .unwrap();
        let left = &data[comma_idx + 1..second_comma];
        let right = &data[second_comma + 1..data.len() - 1];

        let mut result = TreeNode::new(val);
        result.left = Self::deserialize_(left);
        result.right = Self::deserialize_(right);
        Some(Rc::new(RefCell::new(result)))
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */
// << ---------------- Code below here is only for local use ---------------- >>
use cargo_leet::TreeNode;

#[cfg(test)]
mod tests {
    use super::*;
    use cargo_leet::TreeRoot;
    use rstest::rstest;

    #[rstest]
    #[case(TreeRoot::from("[1,2,3,null,null,4,5]").into())]
    #[case(TreeRoot::from("[]").into())]
    #[case(TreeRoot::from("[1,2,3,null,null,-584,5]").into())]
    #[case(TreeRoot::from("[1,2,1000,null,null,-999,-1000]").into())]
    fn case(#[case] root: Option<Rc<RefCell<TreeNode>>>) {
        let codec = Codec::new();
        let expected = root.clone();
        let str_repr = codec.serialize(root);
        let actual = codec.deserialize(str_repr);
        assert_eq!(actual, expected);
    }
}
