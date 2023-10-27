//! Solution for https://leetcode.com/problems/rotate-list
//! 61. Rotate List

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
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() || k <= 0 {
            return head;
        }

        // Calculate length of list
        let mut n = 0;
        let mut curr = &head;
        while let Some(node) = curr {
            n += 1;
            curr = &node.next;
        }

        // Calculate how much we actually need to rotate
        let mut shift = n - (k % n);
        if shift == 0 {
            return head;
        }

        // Find new head for the list
        let mut new_head = &mut head;
        while let Some(node) = new_head {
            new_head = &mut node.as_mut().next;
            shift -= 1;
            if shift == 0 {
                break;
            }
        }
        let mut result = new_head.take();
        debug_assert!(result.is_some());

        // Find tail of remaining list to add the original head to
        let mut tail = &mut result;
        loop {
            let node = tail
                .as_mut()
                .expect("We should not have fallen off the end");
            if node.next.is_some() {
                // Not at the end take a step
                tail = &mut node.next;
            } else {
                // Found the end
                node.next = head;
                break;
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
    #[case(ListHead::from(vec![1,2,3,4,5]).into(), 2, ListHead::from(vec![4,5,1,2,3]).into())]
    #[case(ListHead::from(vec![0,1,2]).into(), 4, ListHead::from(vec![2,0,1]).into())]
    fn case(
        #[case] head: Option<Box<ListNode>>,
        #[case] k: i32,
        #[case] expected: Option<Box<ListNode>>,
    ) {
        let actual = Solution::rotate_right(head, k);
        assert_eq!(actual, expected);
    }
}
