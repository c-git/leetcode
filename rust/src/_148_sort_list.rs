//! Solution for https://leetcode.com/problems/sort-list
//! 148. Sort List

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
    /// Did merge sort at the suggestion of algomaster.io from pattern column
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // Count list length to know how to split
        let mut length = 0;
        let mut curr = &head;
        while let Some(next) = curr.as_ref() {
            length += 1;
            curr = &next.next;
        }
        Self::sort_list_(head, length)
    }

    fn sort_list_(mut head: Option<Box<ListNode>>, length: usize) -> Option<Box<ListNode>> {
        if length <= 1 {
            return head;
        }

        let left_len = length / 2;
        let right_len = length - left_len;

        // Find left tail
        let mut left_tail = head.as_mut().unwrap();
        for _ in 0..left_len - 1 {
            left_tail = left_tail.next.as_mut().unwrap();
        }
        let right_head = left_tail.next.take();

        let mut left_sorted = Self::sort_list_(head, left_len);
        let mut right_sorted = Self::sort_list_(right_head, right_len);

        // Merge sorted halves
        let mut result = None;
        let mut curr_tail = &mut result;
        while let (Some(left), Some(right)) = (left_sorted.as_ref(), right_sorted.as_ref()) {
            if left.val < right.val {
                *curr_tail = left_sorted;
                left_sorted = curr_tail.as_mut().unwrap().next.take();
            } else {
                // Take from right side
                *curr_tail = right_sorted;
                right_sorted = curr_tail.as_mut().unwrap().next.take();
            }
            curr_tail = &mut curr_tail.as_mut().unwrap().next;
        }

        // Append unmerged remainder
        debug_assert!(
            left_sorted.is_some() ^ right_sorted.is_some(),
            "Both CANNOT finish at the same time so one must be empty and the other not empty"
        );
        if left_sorted.is_some() {
            *curr_tail = left_sorted;
        } else {
            // Right is the one that's not empty
            debug_assert!(right_sorted.is_some());
            *curr_tail = right_sorted;
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
    #[case(ListHead::from(vec![4,2,1,3]).into(), ListHead::from(vec![1,2,3,4]).into())]
    #[case(ListHead::from(vec![-1,5,3,4,0]).into(), ListHead::from(vec![-1,0,3,4,5]).into())]
    #[case(ListHead::from(vec![]).into(), ListHead::from(vec![]).into())]
    fn case(#[case] head: Option<Box<ListNode>>, #[case] expected: Option<Box<ListNode>>) {
        let actual = Solution::sort_list(head);
        assert_eq!(actual, expected);
    }
}
