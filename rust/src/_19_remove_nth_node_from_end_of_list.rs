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
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut node_values = vec![];

        // Copy all nodes
        while let Some(node) = head {
            node_values.push(node.val);
            head = node.next;
        }

        let target_index = node_values.len() - n as usize;

        head = None; // Set tail of list
        for (i, &val) in node_values.iter().enumerate().rev() {
            if i != target_index {
                let mut new_node = ListNode::new(val);
                new_node.next = head;
                head = Some(Box::new(new_node));
            }
        }

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
