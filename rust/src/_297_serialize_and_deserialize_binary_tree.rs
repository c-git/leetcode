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
        Self {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if let Some(root) = root {
            format!(
                "({},{},{})",
                root.borrow().val,
                self.serialize(root.borrow().left.clone()),
                self.serialize(root.borrow().right.clone())
            )
        } else {
            "()".to_string()
        }
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        Self::deserialize_str(&data)
    }
    fn deserialize_str(data: &str) -> Option<Rc<RefCell<TreeNode>>> {
        debug_assert!(data.len() >= 2);

        if data.len() == 2 {
            debug_assert_eq!(data, "()");
            None
        } else {
            let mut state = LookingFor::Value;
            let mut start = 1;
            let mut open_bracket_count = 0;
            for (i, c) in data.char_indices() {
                match &state {
                    LookingFor::Value => {
                        if c == ',' {
                            let val = data[start..i].parse().expect("should be a number");
                            state = LookingFor::Left { val };
                            start = i + 1;
                        }
                    }
                    LookingFor::Left { val } => {
                        if start == i {
                            debug_assert_eq!(c, '(');
                            open_bracket_count = 1;
                            continue;
                        }
                        match c {
                            '(' => open_bracket_count += 1,
                            ')' => open_bracket_count -= 1,
                            _ => {}
                        }
                        if open_bracket_count == 0 {
                            let left = Self::deserialize_str(&data[start..=i]);
                            state = LookingFor::Right { val: *val, left };
                            start = i + 2;
                        }
                    }
                    LookingFor::Right { val, left } => {
                        if start > i {
                            debug_assert_eq!(c, ',');
                            continue;
                        }
                        if start == i {
                            debug_assert_eq!(c, '(');
                            open_bracket_count = 1;
                            continue;
                        }
                        match c {
                            '(' => open_bracket_count += 1,
                            ')' => open_bracket_count -= 1,
                            _ => {}
                        }
                        if open_bracket_count == 0 {
                            debug_assert_eq!(i, data.len() - 2,);
                            let right = Self::deserialize_str(&data[start..=i]);
                            state = LookingFor::Complete {
                                val: *val,
                                left: left.clone(),
                                right,
                            };
                            start = i + 1;
                        }
                    }
                    LookingFor::Complete { .. } => {
                        debug_assert_eq!(c, ')');
                    }
                }
            }
            debug_assert!(matches!(state, LookingFor::Complete { .. }));
            if let LookingFor::Complete { val, left, right } = state {
                Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
            } else {
                unreachable!("failed to find all data to deserialize")
            }
        }
    }
}

/// Stores state of previously collected info
enum LookingFor {
    Value,
    Left {
        val: i32,
    },
    Right {
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
    },
    Complete {
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    },
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
