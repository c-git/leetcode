//! Solution for https://leetcode.com/problems/swap-nodes-in-pairs
//! 24. Swap Nodes in Pairs

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
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // Swap first two nodes
        let mut old_head = head?;
        head = old_head.next.take();
        let mut curr;
        if let Some(node) = head.as_mut() {
            old_head.next = node.next.take();
            node.next = Some(old_head);
            curr = &mut node.next.as_mut().unwrap().next;
        } else {
            return Some(old_head);
        }

        while curr.is_some() && curr.as_ref().unwrap().next.is_some() {
            let mut temp = curr.take();
            *curr = temp.as_mut().unwrap().next.take();
            temp.as_mut().unwrap().next = curr.as_mut().unwrap().next.take();
            curr.as_mut().unwrap().next = temp;
            curr = &mut curr.as_mut().unwrap().next.as_mut().unwrap().next;
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
    #[case(ListHead::from(vec![1,2,3,4]).into(), ListHead::from(vec![2,1,4,3]).into())]
    #[case(ListHead::from(vec![]).into(), ListHead::from(vec![]).into())]
    #[case(ListHead::from(vec![1]).into(), ListHead::from(vec![1]).into())]
    #[case(ListHead::from(vec![1,2,3]).into(), ListHead::from(vec![2,1,3]).into())]
    fn case(#[case] head: Option<Box<ListNode>>, #[case] expected: Option<Box<ListNode>>) {
        let actual = Solution::swap_pairs(head);
        assert_eq!(actual, expected);
    }
}
