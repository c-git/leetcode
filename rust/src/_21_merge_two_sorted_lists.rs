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
        let mut result_head: Option<Box<ListNode>> = None;
        let mut result_current = &mut result_head;

        while list1.is_some() || list2.is_some() {
            match (list1, list2) {
                (None, None) => {
                    unreachable!("Should not be reachable by exit condition for while loop")
                }
                (None, Some(node2)) => {
                    if let Some(curr) = result_current {
                        curr.next = Some(node2);
                    } else {
                        *result_current = Some(node2);
                    }
                    list1 = None;
                    list2 = None; // node2 moved into result
                }
                (Some(node1), None) => {
                    if let Some(curr) = result_current {
                        curr.next = Some(node1);
                    } else {
                        *result_current = Some(node1);
                    }
                    list1 = None; // node1 moved into result
                    list2 = None;
                }
                (Some(mut node1), Some(mut node2)) => {
                    if node1.val > node2.val {
                        // Make sure node1 is always the smaller one
                        std::mem::swap(&mut node1, &mut node2);
                    }
                    list1 = node1.next.take();
                    list2 = Some(node2);
                    if let Some(curr) = result_current {
                        curr.next = Some(node1);
                        result_current = &mut curr.next;
                    } else {
                        *result_current = Some(node1);
                    }
                }
            }
        }
        result_head
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
