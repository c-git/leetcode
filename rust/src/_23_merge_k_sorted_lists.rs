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

use std::collections::BinaryHeap;

struct HeapItem {
    head: Box<ListNode>,
}

impl Ord for HeapItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.head.val.cmp(&self.head.val)
    }
}

impl PartialOrd for HeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.head.val.partial_cmp(&self.head.val)
    }
}

impl PartialEq for HeapItem {
    fn eq(&self, other: &Self) -> bool {
        self.head.val == other.head.val
    }
}

impl Eq for HeapItem {}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut result = None;

        // Collect lists into a heap
        let mut queue = BinaryHeap::with_capacity(lists.len());
        for head in lists.into_iter().flatten() {
            // flatten removes any None values
            queue.push(HeapItem { head });
        }

        // Pull from heap until empty
        let mut node = &mut result;
        while let Some(smallest_node) = queue.pop() {
            *node = Some(Box::new(ListNode::new(smallest_node.head.val)));
            node = &mut node.as_mut().unwrap().next;
            if let Some(next) = smallest_node.head.next {
                queue.push(HeapItem { head: next })
            }
        }

        result
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
        let input = vec![vec![1, 4, 5], vec![1, 3, 4], vec![2, 6]];
        let input: Vec<ListHead> = input.into_iter().map(|x| x.into()).collect();
        let expected: ListHead = vec![1, 1, 2, 3, 4, 4, 5, 6].into();
        let actual = Solution::merge_k_lists(input.into_iter().map(|x| x.into()).collect());
        assert_eq!(actual, expected.into());
    }

    #[test]
    fn case2() {
        let input = vec![vec![]];
        let input: Vec<ListHead> = input.into_iter().map(|x| x.into()).collect();
        let expected: ListHead = vec![].into();
        let actual = Solution::merge_k_lists(input.into_iter().map(|x| x.into()).collect());
        assert_eq!(actual, expected.into());
    }

    #[test]
    fn case3() {
        let input: Vec<Vec<i32>> = vec![];
        let input: Vec<ListHead> = input.into_iter().map(|x| x.into()).collect();
        let expected: ListHead = vec![].into();
        let actual = Solution::merge_k_lists(input.into_iter().map(|x| x.into()).collect());
        assert_eq!(actual, expected.into());
    }

    #[test]
    fn case4() {
        let input = vec![vec![1, 4, 5], vec![1, 3, 4], vec![]];
        let input: Vec<ListHead> = input.into_iter().map(|x| x.into()).collect();
        let expected: ListHead = vec![1, 1, 3, 4, 4, 5].into();
        let actual = Solution::merge_k_lists(input.into_iter().map(|x| x.into()).collect());
        assert_eq!(actual, expected.into());
    }
}
