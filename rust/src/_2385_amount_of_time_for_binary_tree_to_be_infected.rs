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
use std::fmt::Debug;
use std::rc::Rc;

struct ResultInfo {
    height: usize,
    /// Height of start node if it exists
    start_height_from_top: Option<usize>,
    /// If the start node exits max time for infection MUST also be known
    time: Option<usize>,
}

impl Debug for ResultInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{h: {}, s: {:?} t: {:?}}}",
            self.height, self.start_height_from_top, self.time
        )
    }
}
impl Solution {
    pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
        Self::check_node(&root, start)
            .time
            .expect("Constraints say start must exist so this must also exist") as i32
    }

    fn check_node(root: &Option<Rc<RefCell<TreeNode>>>, start: i32) -> ResultInfo {
        if let Some(node) = root {
            let node = node.as_ref().borrow();
            let left = Self::check_node(&node.left, start);
            let right = Self::check_node(&node.right, start);
            let height = left.height.max(right.height) + 1;
            let start_height_from_top = match (
                left.start_height_from_top,
                right.start_height_from_top,
                node.val == start,
            ) {
                (None, None, true) => Some(0),
                (None, None, false) => None,
                (Some(_), Some(_), false) | (_, _, true) => {
                    unreachable!("All values in true should be unique")
                }
                (Some(x), None, false) | (None, Some(x), false) => Some(x + 1),
            };

            let time = if let Some(distance_to_start) = start_height_from_top {
                match (left.time, right.time) {
                    (None, None) => {
                        debug_assert!(
                            node.val == start,
                            "This case should only happen when we just found the start value"
                        );
                        Some(height - 1) // Minus 1 because this node starts off infected
                    }
                    (None, Some(x)) => Some(x.max(left.height + distance_to_start)),
                    (Some(x), None) => Some(x.max(right.height + distance_to_start)),
                    (Some(_), Some(_)) => {
                        unreachable!("Both sides should not have a time because time is only present when start is found")
                    }
                }
            } else {
                None
            };
            let result = ResultInfo {
                height,
                start_height_from_top,
                time,
            };

            #[cfg(debug_assertions)]
            println!(
                "node: {} result = {result:?} left_h: {:?} right_h: {:?}",
                node.val, left.height, right.height
            );

            result
        } else {
            ResultInfo {
                height: 0,
                start_height_from_top: None,
                time: None,
            }
        }
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;
use cargo_leet::TreeNode;

#[cfg(test)]
mod tests {
    use super::*;
    use cargo_leet::TreeRoot;

    use rstest::rstest;

    #[rstest]
    #[case(TreeRoot::from("[1,5,3,null,4,10,6,9,2]").into(), 3, 4)]
    #[case(TreeRoot::from("[1]").into(), 1, 0)]
    #[case(TreeRoot::from("[1,2,null,3,null,4,null,5]").into(), 1, 4)]
    #[case(TreeRoot::from("[1,2,null,3,null,4,null,5]").into(), 2, 3)]
    #[case(TreeRoot::from("[1,2,null,3,null,4,null,5]").into(), 4, 3)]
    #[case(TreeRoot::from("[1,2,null,3,null,4,null,5]").into(), 4, 3)]
    #[case(TreeRoot::from("[5,2,3,4,null,null,null,1]").into(), 4, 3)]
    fn case(
        #[case] root: Option<Rc<RefCell<TreeNode>>>,
        #[case] start: i32,
        #[case] expected: i32,
    ) {
        let actual = Solution::amount_of_time(root, start);
        assert_eq!(actual, expected);
    }

    #[test]
    #[ignore = "Cannot be solved with recursive solution as you'll get stack overflow, will also need to forget input otherwise std::mem::drop will stack overflow"]
    fn large_input() {
        let file_value = std::fs::read_to_string("large_inputs/2385_test_case 78.txt").unwrap();
        let root = TreeRoot::from(&file_value[..]);
        let start = 100000;
        let expected = 99999;
        let actual = Solution::amount_of_time(root.into(), start);
        assert_eq!(actual, expected);
    }
}
