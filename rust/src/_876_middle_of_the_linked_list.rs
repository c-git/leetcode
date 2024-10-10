//! Solution for https://leetcode.com/problems/middle-of-the-linked-list
//! 876. Middle of the Linked List

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
    pub fn middle_node(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut n = 0;
        let mut curr = &head;
        while let Some(node) = curr {
            n += 1;
            curr = &node.next;
        }
        for _ in 0..n / 2 {
            head = head.unwrap().next.take();
        }
        head
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
    #[case(ListHead::from(vec![1,2,3,4,5]).into(), ListHead::from(vec![3,4,5]).into())]
    #[case(ListHead::from(vec![1,2,3,4,5,6]).into(), ListHead::from(vec![4,5,6]).into())]
    fn case(#[case] head: Option<Box<ListNode>>, #[case] expected: Option<Box<ListNode>>) {
        let actual = Solution::middle_node(head);
        assert_eq!(actual, expected);
    }
}
