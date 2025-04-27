//! Solution for https://leetcode.com/problems/path-sum-iii
//! 437. Path Sum III

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
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    /// Based on https://www.youtube.com/watch?v=zraEXluZLj0
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut result = 0;
        let mut previous_segment_sums = HashMap::new();
        previous_segment_sums.insert(0i32, 1u16); // Subtracting nothing is always an option
        Self::path_sum_(root, target_sum, 0, &mut previous_segment_sums, &mut result);
        result
    }

    pub fn path_sum_(
        root: Option<Rc<RefCell<TreeNode>>>,
        target_sum: i32,
        mut curr_path_sum: i32,
        previous_segment_sums: &mut HashMap<i32, u16>,
        result: &mut i32,
    ) {
        let Some(root) = root else {
            return;
        };
        let mut root = root.borrow_mut();

        curr_path_sum += root.val;
        if let Some(&way_count) = previous_segment_sums.get(&(curr_path_sum - target_sum)) {
            // Add the number of segments we can remove that would give us our target
            *result += way_count as i32;
        }

        // Add this segment as an option for removal
        *previous_segment_sums.entry(curr_path_sum).or_default() += 1;

        Self::path_sum_(
            root.left.take(),
            target_sum,
            curr_path_sum,
            previous_segment_sums,
            result,
        );
        Self::path_sum_(
            root.right.take(),
            target_sum,
            curr_path_sum,
            previous_segment_sums,
            result,
        );

        // Remove this segment as possible
        *previous_segment_sums
            .get_mut(&curr_path_sum)
            .expect("should have been added above with or_default") -= 1;
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
    #[case(TreeRoot::from("[10,5,-3,3,2,null,11,3,-2,null,1]").into(), 8, 3)]
    #[case(TreeRoot::from("[5,4,8,11,null,13,4,7,2,null,null,5,1]").into(), 22, 3)]
    fn case(
        #[case] root: Option<Rc<RefCell<TreeNode>>>,
        #[case] target_sum: i32,
        #[case] expected: i32,
    ) {
        let actual = Solution::path_sum(root, target_sum);
        assert_eq!(actual, expected);
    }
}
