//! Solution for https://leetcode.com/problems/remove-nth-node-from-end-of-list
//! 19. Remove Nth Node From End of List

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
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        // Unable to use two pointers at the same time with the second being n nodes behind as that would a second mutable borrow is not allowed
        let mut count = 0;
        let mut curr = &head;
        while let Some(node) = curr {
            count += 1;
            curr = &node.next;
        }

        let steps_needed = count - n;
        let mut curr = &mut head;
        for _ in 0..steps_needed {
            curr = &mut curr.as_mut().unwrap().next;
        }
        let next = curr.as_mut().unwrap().next.take();
        *curr = next;

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
    #[case(ListHead::from(vec![1,2,3,4,5]).into(), 2, ListHead::from(vec![1,2,3,5]).into())]
    #[case(ListHead::from(vec![1]).into(), 1, ListHead::from(vec![]).into())]
    #[case(ListHead::from(vec![1,2]).into(), 1, ListHead::from(vec![1]).into())]
    fn case(
        #[case] head: Option<Box<ListNode>>,
        #[case] n: i32,
        #[case] expected: Option<Box<ListNode>>,
    ) {
        let actual = Solution::remove_nth_from_end(head, n);
        assert_eq!(actual, expected);
    }
}
