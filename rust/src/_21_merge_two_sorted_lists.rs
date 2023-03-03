use crate::helper::{ListHead, ListNode};

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
struct Solution;
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        todo!()
    }
}

#[test]
fn case1() {
    let l1: ListHead = vec![1, 2, 4].into();
    let l2: ListHead = vec![1, 3, 4].into();
    let expected: ListHead = vec![1, 1, 2, 3, 4, 4].into();

    let actual = Solution::merge_two_lists(l1.into(), l2.into());
    assert_eq!(expected, actual.into());
}

#[test]
fn case2() {
    let l1: ListHead = vec![].into();
    let l2: ListHead = vec![].into();
    let expected: ListHead = vec![].into();

    let actual = Solution::merge_two_lists(l1.into(), l2.into());
    assert_eq!(expected, actual.into());
}

#[test]
fn case3() {
    let l1: ListHead = vec![].into();
    let l2: ListHead = vec![0].into();
    let expected: ListHead = vec![0].into();

    let actual = Solution::merge_two_lists(l1.into(), l2.into());
    assert_eq!(expected, actual.into());
}
