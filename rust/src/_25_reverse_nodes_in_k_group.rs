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
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        // Source: sak96 (with modifications)
        // 1 means no swap
        if k == 1 {
            return head;
        }

        let mut node_opt = &mut head;
        // Move forward k nodes
        for _ in 0..k {
            if let Some(node) = node_opt {
                node_opt = &mut node.next;
            } else {
                return head;
            }
        }

        // separate out the unprocessed nodes
        let unprocessed = node_opt.take();

        // reverse the rest of the list
        let mut rest: Option<Box<ListNode>> = Self::reverse_k_group(unprocessed, k);

        // Attach the reversed nodes in this group to the front
        while let Some(mut node1) = head.take() {
            head = node1.next.take();
            node1.next = rest;
            rest = Some(node1);
        }

        rest
    }
}

use crate::helper::ListNode;
struct Solution;
#[cfg(test)]
mod tests {
    use crate::helper::ListHead;

    use super::*;

    #[test]
    fn case1() {
        let head: ListHead = vec![1, 2, 3, 4, 5].into();
        let k = 2;
        let expected: ListHead = vec![2, 1, 4, 3, 5].into();
        let actual = Solution::reverse_k_group(head.into(), k);
        assert_eq!(actual, expected.into());
    }

    #[test]
    fn case2() {
        let head: ListHead = vec![1, 2, 3, 4, 5].into();
        let k = 3;
        let expected: ListHead = vec![3, 2, 1, 4, 5].into();
        let actual = Solution::reverse_k_group(head.into(), k);
        assert_eq!(actual, expected.into());
    }

    #[test]
    fn case3() {
        let head: ListHead = vec![1].into();
        let k = 1;
        let expected: ListHead = vec![1].into();
        let actual = Solution::reverse_k_group(head.into(), k);
        assert_eq!(actual, expected.into());
    }

    #[test]
    fn case4() {
        let head: ListHead = vec![1, 2, 3, 4, 5, 6].into();
        let k = 1;
        let expected: ListHead = vec![1, 2, 3, 4, 5, 6].into();
        let actual = Solution::reverse_k_group(head.into(), k);
        assert_eq!(actual, expected.into());
    }

    #[test]
    fn case5() {
        let head: ListHead = vec![1, 2, 3, 4, 5, 6].into();
        let k = 2;
        let expected: ListHead = vec![2, 1, 4, 3, 6, 5].into();
        let actual = Solution::reverse_k_group(head.into(), k);
        assert_eq!(actual, expected.into());
    }
}
