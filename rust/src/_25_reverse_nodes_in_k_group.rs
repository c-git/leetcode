//! Solution for https://leetcode.com/problems/reverse-nodes-in-k-group
//! 25. Reverse Nodes in k-Group

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
    // Source: sak96 - Converted to iterative
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        // 1 means no swap
        if k == 1 {
            return head;
        }

        let mut unprocessed = &mut head;
        loop {
            let mut group_start = unprocessed.take();

            // Move forward k nodes
            let mut node_opt = &mut group_start;
            for _ in 0..k {
                if let Some(node) = node_opt {
                    node_opt = &mut node.next;
                } else {
                    *unprocessed = group_start;
                    return head;
                }
            }

            // separate out the rest of the nodes
            let mut rest = node_opt.take();

            // Reverse this group
            while let Some(mut node) = group_start.take() {
                group_start = node.next.take();
                node.next = rest;
                rest = Some(node);
            }

            *unprocessed = rest;

            // Move over group that was processed
            for _ in 0..k {
                unprocessed = &mut unprocessed.as_mut().unwrap().next;
            }
        }
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
    #[case(ListHead::from(vec![1,2,3,4,5]).into(), 2, ListHead::from(vec![2,1,4,3,5]).into())]
    #[case(ListHead::from(vec![1,2,3,4,5]).into(), 3, ListHead::from(vec![3,2,1,4,5]).into())]
    fn case(
        #[case] head: Option<Box<ListNode>>,
        #[case] k: i32,
        #[case] expected: Option<Box<ListNode>>,
    ) {
        let actual = Solution::reverse_k_group(head, k);
        assert_eq!(actual, expected);
    }
}
