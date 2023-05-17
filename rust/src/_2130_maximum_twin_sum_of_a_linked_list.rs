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
    pub fn pair_sum(mut head: Option<Box<ListNode>>) -> i32 {
        let mut result = 0; // Initialized to 0 because, values are all positive and min of 2

        // Calculate n
        let mut node_opt = &head;
        let mut n = 0;
        while let Some(node) = node_opt {
            n += 1;
            node_opt = &node.next;
        }

        // Move right pointer to first node in right side
        let mut left = &mut head;
        for _ in 0..n / 2 - 1 {
            left = &mut left.as_mut().unwrap().next;
        }

        // Flip second part of list
        let mut rest = None;
        let mut right = left.as_mut().unwrap().next.take();
        for _ in 0..n / 2 {
            let next = right.as_mut().unwrap().next.take();
            right.as_mut().unwrap().next = rest;
            rest = right;
            right = next;
        }

        // Walk lists and compare sums
        let mut left = head; // Use left as nodes from head
        right = rest; // Move rest into the variable for the right side of the twins (The flipped second half of the list)
        while let (Some(node1), Some(node2)) = (left, right) {
            let sum = node1.val + node2.val;
            if sum > result {
                result = sum;
            }
            left = node1.next;
            right = node2.next;
            debug_assert!(
                left.is_none() == right.is_none(),
                "Lists should be equal length and finish at the same time"
            );
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
        let input: ListHead = vec![5, 4, 2, 1].into();
        let expected = 6;
        let actual = Solution::pair_sum(input.into());
        assert_eq!(actual, expected);
    }

    #[test]
    fn case2() {
        let input: ListHead = vec![4, 2, 2, 3].into();
        let expected = 7;
        let actual = Solution::pair_sum(input.into());
        assert_eq!(actual, expected);
    }

    #[test]
    fn case3() {
        let input: ListHead = vec![1, 100000].into();
        let expected = 100001;
        let actual = Solution::pair_sum(input.into());
        assert_eq!(actual, expected);
    }
}
