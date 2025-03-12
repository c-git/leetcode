//! Solution for https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii
//! 82. Remove Duplicates from Sorted List II

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
        let mut candidate: Option<Box<ListNode>> = None;
        let mut is_candidate_duplicate = false;
        while let Some(mut node) = head {
            head = node.next.take();
            match (is_candidate_duplicate, candidate.as_ref()) {
                (true, None) => {
                    unreachable!("We should never not have a candidate and that candidate that we do not have is a duplicate")
                }
                (true, Some(candidate_node)) => {
                    if candidate_node.val != node.val {
                        // Discard current candidate as it's still part of the duplicate set of
                        // nodes and create new candidate
                        candidate = Some(node);
                        is_candidate_duplicate = false;
                    } else {
                        // Current node is discarded
                    }
                }
                (false, None) => {
                    candidate = Some(node);
                }
                (false, Some(candidate_node)) => {
                    if candidate_node.val == node.val {
                        // Record as duplicate and discard current node
                        is_candidate_duplicate = true;
                    } else {
                        // This candidate is not duplicated let's add it to the output
                        if let Some(tail_node) = tail {
                            tail_node.next = candidate.take();
                            tail = &mut tail_node.next.as_mut().expect("set on line above").next;
                        } else {
                            *tail = candidate.take();
                            tail = &mut tail.as_mut().expect("just set to candidate").next;
                        }
                        candidate = Some(node);
                        is_candidate_duplicate = false;
                    }
                }
            }
        }
        if !is_candidate_duplicate {
            *tail = candidate.take();
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
    #[case(ListHead::from(vec![1,2,3,3,4,4,5]).into(), ListHead::from(vec![1,2,5]).into())]
    #[case(ListHead::from(vec![1,1,1,2,3]).into(), ListHead::from(vec![2,3]).into())]
    #[case(ListHead::from(vec![1,1,1,2,2,3,3]).into(), ListHead::from(vec![]).into())]
    fn case(#[case] head: Option<Box<ListNode>>, #[case] expected: Option<Box<ListNode>>) {
        let actual = Solution::delete_duplicates(head);
        assert_eq!(actual, expected);
    }
}
