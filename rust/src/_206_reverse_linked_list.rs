//! Solution for https://leetcode.com/problems/reverse-linked-list
//! 206. Reverse Linked List

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
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result = None;
        let new_head = &mut result;
        while let Some(mut node) = head {
            head = node.next.take();
            node.next = new_head.take();
            *new_head = Some(node);
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
    #[case(ListHead::from(vec![1,2,3,4,5]).into(), ListHead::from(vec![5,4,3,2,1]).into())]
    #[case(ListHead::from(vec![1,2]).into(), ListHead::from(vec![2,1]).into())]
    #[case(ListHead::from(vec![]).into(), ListHead::from(vec![]).into())]
    fn case(#[case] head: Option<Box<ListNode>>, #[case] expected: Option<Box<ListNode>>) {
        let actual = Solution::reverse_list(head);
        assert_eq!(actual, expected);
    }
}
