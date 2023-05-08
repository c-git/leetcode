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
    // Based on solution by Conrad Ludgate
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let len = len(&head) as i32; // Get length of list to calculate how far from the front
        remove_nth_from_start(head, len - n)
    }
}

pub fn remove_nth_from_start(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut head = head?;
    if n == 0 {
        head.next
    } else {
        head.next = remove_nth_from_start(head.next, n - 1);
        Some(head)
    }
}

pub fn len(mut head: &Option<Box<ListNode>>) -> usize {
    let mut len = 0;
    while let Some(h) = head.as_ref() {
        head = &h.next;
        len += 1;
    }
    len
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
        let n = 2;
        let expected: ListHead = vec![1, 2, 3, 5].into();
        let actual = Solution::remove_nth_from_end(head.into(), n);
        assert_eq!(actual, expected.into());
        panic!("Intentional");
    }

    #[test]
    fn case2() {
        let head: ListHead = vec![1].into();
        let n = 1;
        let expected: ListHead = vec![].into();
        let actual = Solution::remove_nth_from_end(head.into(), n);
        assert_eq!(actual, expected.into());
    }

    #[test]
    fn case3() {
        let head: ListHead = vec![1, 2].into();
        let n = 1;
        let expected: ListHead = vec![1].into();
        let actual = Solution::remove_nth_from_end(head.into(), n);
        assert_eq!(actual, expected.into());
    }
}
