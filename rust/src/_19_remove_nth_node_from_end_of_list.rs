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
        let len = Self::get_list_length(&head);

        // Walk from front and collect values to rebuild list without undesirable node
        let target_length = len - n as usize; // Doesn't have space for the undesirable node
        let mut values = Vec::with_capacity(target_length);
        let mut rest_of_list = head;
        while values.len() < target_length {
            let node = rest_of_list.expect("Target Length must be less than full length");
            values.push(node.val);
            rest_of_list = node.next;
        }

        // Rebuild list skipping the undesirable value
        rest_of_list = rest_of_list.expect("This is the node to skip").next; // Skipped here
        for val in values.iter().rev() {
            let mut new_node = ListNode::new(*val);
            new_node.next = rest_of_list;
            rest_of_list = Some(Box::new(new_node));
        }

        rest_of_list
    }

    fn get_list_length(mut head: &Option<Box<ListNode>>) -> usize {
        let mut len = 0;
        while let Some(h) = head.as_ref() {
            head = &h.next;
            len += 1;
        }
        len
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
        let n = 2;
        let expected: ListHead = vec![1, 2, 3, 5].into();
        let actual = Solution::remove_nth_from_end(head.into(), n);
        assert_eq!(actual, expected.into());
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
