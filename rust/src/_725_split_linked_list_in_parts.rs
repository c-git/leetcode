//! Solution for https://leetcode.com/problems/split-linked-list-in-parts
//! 725. Split Linked List in Parts

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
    pub fn split_list_to_parts(
        mut head: Option<Box<ListNode>>,
        k: i32,
    ) -> Vec<Option<Box<ListNode>>> {
        let mut result = Vec::with_capacity(k as usize);
        let mut n = 0;
        let mut curr = head.as_ref();
        while let Some(node) = curr {
            n += 1;
            curr = node.next.as_ref();
        }
        let count_each = n / k;
        let mut extra = n - count_each * k;
        for _ in 0..k {
            let mut new_head = head.take();
            let mut curr = new_head.as_mut();
            // Now move forward and cut list
            let mut i = 1;
            if extra > 0 {
                i -= 1;
                extra -= 1;
            }
            while i < count_each {
                let val = curr.as_ref().unwrap().val;
                curr = curr
                    .expect("Based on the calculation this should exist")
                    .next
                    .as_mut();
                i += 1;
            }
            if let Some(node) = curr.as_mut() {
                head = node.next.take();
            }
            result.push(new_head);
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
    #[case(ListHead::from(vec![1,2,3]).into(), 5, vec![ListHead::from(vec![1]).into(),ListHead::from(vec![2]).into(),ListHead::from(vec![3]).into(),ListHead::from(vec![]).into(),ListHead::from(vec![]).into()])]
    #[case(ListHead::from(vec![1,2,3,4,5,6,7,8,9,10]).into(), 3, vec![ListHead::from(vec![1,2,3,4]).into(),ListHead::from(vec![5,6,7]).into(),ListHead::from(vec![8,9,10]).into()])]
    fn case(
        #[case] head: Option<Box<ListNode>>,
        #[case] k: i32,
        #[case] expected: Vec<Option<Box<ListNode>>>,
    ) {
        let actual = Solution::split_list_to_parts(head, k);
        assert_eq!(actual, expected);
    }
}
