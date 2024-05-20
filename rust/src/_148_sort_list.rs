//! Solution for https://leetcode.com/problems/sort-list
//! 148. Sort List

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
    pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // Testing basic bubble sort

        let mut unsorted = true;
        while unsorted {
            unsorted = false;
            let mut next = &mut head;
            while let Some(node) = next.as_mut() {
                if let Some(next_node) = node.next.as_mut() {
                    if node.val > next_node.val {
                        std::mem::swap(&mut node.val, &mut next_node.val);
                        unsorted = true;
                    }
                }
                next = &mut node.next;
            }
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
    #[case(ListHead::from(vec![4,2,1,3]).into(), ListHead::from(vec![1,2,3,4]).into())]
    #[case(ListHead::from(vec![-1,5,3,4,0]).into(), ListHead::from(vec![-1,0,3,4,5]).into())]
    #[case(ListHead::from(vec![]).into(), ListHead::from(vec![]).into())]
    fn case(#[case] head: Option<Box<ListNode>>, #[case] expected: Option<Box<ListNode>>) {
        let actual = Solution::sort_list(head);
        assert_eq!(actual, expected);
    }
}
