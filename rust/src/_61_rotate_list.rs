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
        // Find length of list
        let n = {
            let mut n = 0;
            let mut curr = &head;
            while let Some(node) = curr {
                n += 1;
                curr = &node.next
            }
            n
        };
        if n == 0 {
            return None;
        }

        // Find number of elements we need to walk before we cut
        let walk_distance = n - k % n;
        if walk_distance == 0 {
            return head;
        }
        debug_assert!(walk_distance > 0);

        // Walk list then cut
        let mut curr = &mut head;
        for _ in 1..walk_distance {
            curr = &mut curr
                .as_mut()
                .expect("we already checked the length and should be walking less than that")
                .next;
        }
        let mut new_head = curr
            .as_mut()
            .expect("we should always stop within the list")
            .next
            .take();

        // Find end of cut out piece
        let mut last = new_head
            .as_mut()
            .expect("we should have not run if this is past the list");
        while last.next.is_some() {
            last = last
                .next
                .as_mut()
                .expect("checked it was some before entering the loop");
        }

        // Rejoin parts
        last.next = head;

        // Return new list
        new_head
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
