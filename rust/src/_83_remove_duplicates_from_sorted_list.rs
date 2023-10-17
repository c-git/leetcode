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
        let mut result: Option<Box<ListNode>> = None;
        let mut tail = &mut result;
        while let Some(mut node) = head {
            head = node.next.take();
            let last_value = tail.as_ref().map(|last_node| last_node.as_ref().val);
            match last_value {
                Some(val) if val != node.val => {
                    tail.as_mut()
                        .expect("Should only have value if tail was some")
                        .next = Some(node);
                    tail = &mut tail.as_mut().unwrap().next;
                }
                Some(_) => {} // Drop value as it's same as the previous
                None => *tail = Some(node),
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
    #[case(ListHead::from(vec![1,1,2]).into(), ListHead::from(vec![1,2]).into())]
    #[case(ListHead::from(vec![1,1,2,3,3]).into(), ListHead::from(vec![1,2,3]).into())]
    fn case(#[case] head: Option<Box<ListNode>>, #[case] expected: Option<Box<ListNode>>) {
        let actual = Solution::delete_duplicates(head);
        assert_eq!(actual, expected);
    }
}
