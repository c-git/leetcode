//! Solution for https://leetcode.com/problems/reverse-linked-list-ii
//! 92. Reverse Linked List II

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
    pub fn reverse_between(
        mut head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        // Used a vec to simplify implementation got stuck walking the list mutably to be able to do the split
        if left == right {
            return head;
        }

        let mut values = Vec::with_capacity(right as usize);
        for _ in 1..=right {
            values.push(head.as_ref().unwrap().val);
            head = head.unwrap().next;
        }

        // rebuild list
        for idx in left - 1..right {
            let mut node = ListNode::new(values[idx as usize]);
            node.next = head;
            head = Some(Box::new(node));
        }
        for idx in (0..left - 1).rev() {
            let mut node = ListNode::new(values[idx as usize]);
            node.next = head;
            head = Some(Box::new(node));
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
    #[case(ListHead::from(vec![1,2,3,4,5]).into(), 2, 4, ListHead::from(vec![1,4,3,2,5]).into())]
    #[case(ListHead::from(vec![5]).into(), 1, 1, ListHead::from(vec![5]).into())]
    #[case(ListHead::from(vec![1,2,3,4,5]).into(), 1, 4, ListHead::from(vec![4,3,2,1,5]).into())]
    #[case(ListHead::from(vec![1,2,3,4,5]).into(), 1, 5, ListHead::from(vec![5,4,3,2,1]).into())]
    #[case(ListHead::from(vec![1,2,3,4,5]).into(), 3, 5, ListHead::from(vec![1,2,5,4,3]).into())]
    fn case(
        #[case] head: Option<Box<ListNode>>,
        #[case] left: i32,
        #[case] right: i32,
        #[case] expected: Option<Box<ListNode>>,
    ) {
        let actual = Solution::reverse_between(head, left, right);
        assert_eq!(actual, expected);
    }
}
