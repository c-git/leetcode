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

use std::cmp::Ordering::{Equal, Greater, Less};

impl Solution {
    pub fn swap_nodes(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let k = k as usize;
        let mut fast = &head;
        let mut slow = head.as_ref().expect("Constraint is n >= 1");

        // Move fast to index k
        for _ in 1..k {
            fast = &fast
                .as_ref()
                .expect("Based on constraint in question k <= n")
                .next;
        }

        // Save value at k from start
        let k_from_start_value = fast
            .as_ref()
            .expect("Based on constraint in question k <= n")
            .val;

        // Move fast onto the next node after k from start so it is k ahead of slow which is on 1
        fast = &fast
            .as_ref()
            .expect("Based on constraint in question k <= n")
            .next;

        // Move slow to k before the end
        let mut k_from_end_index = 1;
        while let Some(node) = fast {
            fast = &node.next;
            slow = slow
                .next
                .as_ref()
                .expect("This is behind fast so must exist");
            k_from_end_index += 1;
        }

        // Save value from k from end
        let k_from_end_value = slow.val;

        let (first_index, second_index, first_value, second_value) = match k.cmp(&k_from_end_index)
        {
            Less | Equal => (k, k_from_end_index, k_from_end_value, k_from_start_value),
            Greater => (k_from_end_index, k, k_from_start_value, k_from_end_value),
        };

        // Find first node
        let mut node = &mut head;
        for _ in 1..first_index {
            node = &mut node.as_mut().unwrap().next;
        }
        node.as_mut().expect("At first node").val = first_value;

        // Update second node
        for _ in first_index..second_index {
            node = &mut node.as_mut().unwrap().next;
        }
        node.as_mut().expect("At second node").val = second_value;

        head
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
        let expected: ListHead = vec![1, 4, 3, 2, 5].into();
        let actual = Solution::swap_nodes(head.into(), k);
        assert_eq!(actual, expected.into());
    }

    #[test]
    fn case2() {
        let head: ListHead = vec![7, 9, 6, 6, 7, 8, 3, 0, 9, 5].into();
        let k = 5;
        let expected: ListHead = vec![7, 9, 6, 6, 8, 7, 3, 0, 9, 5].into();
        let actual = Solution::swap_nodes(head.into(), k);
        assert_eq!(actual, expected.into());
    }

    #[test]
    fn case3() {
        let head: ListHead = vec![5].into();
        let k = 1;
        let expected: ListHead = vec![5].into();
        let actual = Solution::swap_nodes(head.into(), k);
        assert_eq!(actual, expected.into());
    }

    #[test]
    fn case4() {
        let head: ListHead = vec![1, 2, 3].into();
        let k = 1;
        let expected: ListHead = vec![3, 2, 1].into();
        let actual = Solution::swap_nodes(head.into(), k);
        assert_eq!(actual, expected.into());
    }

    #[test]
    fn case5() {
        let head: ListHead = vec![1, 2].into();
        let k = 1;
        let expected: ListHead = vec![2, 1].into();
        let actual = Solution::swap_nodes(head.into(), k);
        assert_eq!(actual, expected.into());
    }

    #[test]
    fn case6() {
        let head: ListHead = vec![1, 2].into();
        let k = 2;
        let expected: ListHead = vec![2, 1].into();
        let actual = Solution::swap_nodes(head.into(), k);
        assert_eq!(actual, expected.into());
    }

    #[test]
    fn case7() {
        let head: ListHead = vec![1, 2, 3, 4, 5, 6].into();
        let k = 6;
        let expected: ListHead = vec![6, 2, 3, 4, 5, 1].into();
        let actual = Solution::swap_nodes(head.into(), k);
        assert_eq!(actual, expected.into());
    }

    #[test]
    fn case8() {
        let head: ListHead = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10].into();
        let k = 8;
        let expected: ListHead = vec![1, 2, 8, 4, 5, 6, 7, 3, 9, 10].into();
        let actual = Solution::swap_nodes(head.into(), k);
        assert_eq!(actual, expected.into());
    }
}
