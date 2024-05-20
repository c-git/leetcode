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
    pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // Wanted to test out a bubble sort idea but couldn't figure out how to
        // mutate the later parts of the list while holding onto a reference to the start
        // and decided didn't add value so split up the nodes and sorted them and rejoined them

        // Split list into nodes
        let mut nodes = vec![];
        while let Some(mut node) = head {
            head = node.as_mut().next.take();
            nodes.push(node);
        }

        // Short split list (switched to bubble sort to see if that's why I got TLE)
        let mut unsorted = true;
        while unsorted {
            unsorted = false;
            for i in 0..nodes.len().saturating_sub(1) {
                if nodes[i].val > nodes[i + 1].val {
                    nodes.swap(i, i + 1);
                    unsorted = true;
                }
            }
        }

        // Rejoin list
        let mut result = None;
        for mut node in nodes.into_iter().rev() {
            node.next = result;
            result = Some(node);
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
