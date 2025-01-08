//! Solution for https://leetcode.com/problems/merge-two-sorted-lists
//! 21. Merge Two Sorted Lists

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result = None;
        let mut current = &mut result;
        while list1.is_some() || list2.is_some() {
            match (list1.as_ref(), list2.as_ref()) {
                (None, None) => unreachable!("either must be some to enter loop"),
                (None, Some(_)) => *current = list2.take(),
                (Some(_), None) => *current = list1.take(),
                (Some(x), Some(y)) => {
                    if x.val > y.val {
                        std::mem::swap(&mut list1, &mut list2);
                    }
                    let mut temp = list1.take();
                    list1 = temp.as_mut().expect("just checked it was some").next.take();
                    *current = temp;
                    current = &mut current.as_mut().expect("just checked it was some").next;
                }
            }
        }
        result
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;
use cargo_leet::ListNode;

#[cfg(test)]
mod tests {
    use super::*;
    use cargo_leet::ListHead;

    use rstest::rstest;

    #[rstest]
    #[case(ListHead::from(vec![1,2,4]).into(), ListHead::from(vec![1,3,4]).into(), ListHead::from(vec![1,1,2,3,4,4]).into())]
    #[case(ListHead::from(vec![]).into(), ListHead::from(vec![]).into(), ListHead::from(vec![]).into())]
    #[case(ListHead::from(vec![]).into(), ListHead::from(vec![0]).into(), ListHead::from(vec![0]).into())]
    fn case(
        #[case] list1: Option<Box<ListNode>>,
        #[case] list2: Option<Box<ListNode>>,
        #[case] expected: Option<Box<ListNode>>,
    ) {
        let actual = Solution::merge_two_lists(list1, list2);
        assert_eq!(actual, expected);
    }
}
