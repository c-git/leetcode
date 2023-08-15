//! Solution for https://leetcode.com/problems/partition-list
//! 86. Partition List

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
#[allow(clippy::collapsible_else_if)]
impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut less_head: Option<Box<ListNode>> = None;
        let mut more_head: Option<Box<ListNode>> = None;
        let mut less = &mut less_head;
        let mut more = &mut more_head;

        let mut curr = head;
        while let Some(mut node) = curr {
            curr = node.next.take();
            if node.val < x {
                if let Some(le) = less {
                    le.next = Some(node);
                    less = &mut le.next;
                } else {
                    *less = Some(node);
                }
            } else {
                if let Some(mo) = more {
                    mo.next = Some(node);
                    more = &mut mo.next;
                } else {
                    *more = Some(node);
                }
            }
        }

        if let Some(less_tail) = less {
            less_tail.next = more_head;
        } else {
            less_head = more_head;
        }

        less_head
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
    #[case(ListHead::from(vec![1,4,3,2,5,2]).into(), 3, ListHead::from(vec![1,2,2,4,3,5]).into())]
    #[case(ListHead::from(vec![2,1]).into(), 2, ListHead::from(vec![1,2]).into())]
    fn case(
        #[case] head: Option<Box<ListNode>>,
        #[case] x: i32,
        #[case] expected: Option<Box<ListNode>>,
    ) {
        let actual = Solution::partition(head, x);
        assert_eq!(actual, expected);
    }
}
