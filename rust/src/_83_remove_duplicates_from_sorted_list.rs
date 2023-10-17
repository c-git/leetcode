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
        let mut current = &mut head;
        while let Some(node) = current {
            let next_val = node.as_mut().next.as_mut().map(|x| x.val);
            if let Some(val) = next_val {
                if node.as_ref().val == val {
                    // Same value skip next node
                    node.next = node
                        .as_mut()
                        .next
                        .as_mut()
                        .expect("Should only have value if it exists")
                        .as_mut()
                        .next
                        .take();
                }
            }
            current = &mut node.as_mut().next;
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
    fn case(#[case] head: Option<Box<ListNode>>, #[case] expected: Option<Box<ListNode>>) {
        let actual = Solution::delete_duplicates(head);
        assert_eq!(actual, expected);
    }
}
