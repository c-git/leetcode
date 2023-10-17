//! Solution for https://leetcode.com/problems/remove-duplicates-from-sorted-list
//! 83. Remove Duplicates from Sorted List

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
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.as_ref()?; // Exits early if head is none

        let mut current = &mut head;
        let mut last_val = current.as_mut().unwrap().val;
        current = &mut current.as_mut().unwrap().next;
        while current.is_some() {
            let this_value = current.as_ref().unwrap().as_ref().val;
            if this_value == last_val {
                // Same value skip next node
                *current = current.as_mut().unwrap().as_mut().next.take();
            } else {
                current = &mut current.as_mut().unwrap().as_mut().next;
            }
            last_val = this_value;
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
    #[case(ListHead::from(vec![1,1,2]).into(), ListHead::from(vec![1,2]).into())]
    #[case(ListHead::from(vec![1,1,2,3,3]).into(), ListHead::from(vec![1,2,3]).into())]
    #[case(ListHead::from(vec![1,1,1]).into(), ListHead::from(vec![1]).into())]
    fn case(#[case] head: Option<Box<ListNode>>, #[case] expected: Option<Box<ListNode>>) {
        let actual = Solution::delete_duplicates(head);
        assert_eq!(actual, expected);
    }
}
