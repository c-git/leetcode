use crate::helper::ListNode;

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
        let mut result_head: Option<Box<ListNode>> = None;
        let mut result_current = &mut result_head;

        // Change variables to be mutable to only do splicing and not allocate new nodes
        let mut list1 = list1;
        let mut list2 = list2;
        while list1.is_some() || list2.is_some() {
            match (list1, list2) {
                (None, None) => {
                    unreachable!("Should not be reachable by exit condition for while loop")
                }
                (None, Some(node2)) => {
                    if let Some(curr) = result_current {
                        curr.next = Some(node2);
                    } else {
                        *result_current = Some(node2);
                    }
                    list1 = None;
                    list2 = None; // node2 moved into result
                }
                (Some(node1), None) => {
                    if let Some(curr) = result_current {
                        curr.next = Some(node1);
                    } else {
                        *result_current = Some(node1);
                    }
                    list1 = None; // node1 moved into result
                    list2 = None;
                }
                (Some(mut node1), Some(mut node2)) => {
                    if node1.val > node2.val {
                        // Make sure node1 is always the smaller one
                        std::mem::swap(&mut node1, &mut node2);
                    }
                    list1 = node1.next;
                    node1.next = None;
                    list2 = Some(node2);
                    if let Some(curr) = result_current {
                        curr.next = Some(node1);
                        result_current = &mut curr.next;
                    } else {
                        *result_current = Some(node1);
                    }
                }
            }
        }
        result_head
    }
}

#[test]
fn case1() {
    let l1: crate::helper::ListHead = vec![1, 2, 4].into();
    let l2: crate::helper::ListHead = vec![1, 3, 4].into();
    let expected: crate::helper::ListHead = vec![1, 1, 2, 3, 4, 4].into();

    let actual = Solution::merge_two_lists(l1.into(), l2.into());
    assert_eq!(actual, expected.into());
}

#[test]
fn case2() {
    let l1: crate::helper::ListHead = vec![].into();
    let l2: crate::helper::ListHead = vec![].into();
    let expected: crate::helper::ListHead = vec![].into();

    let actual = Solution::merge_two_lists(l1.into(), l2.into());
    assert_eq!(actual, expected.into());
}

#[test]
fn case3() {
    let l1: crate::helper::ListHead = vec![].into();
    let l2: crate::helper::ListHead = vec![0].into();
    let expected: crate::helper::ListHead = vec![0].into();

    let actual = Solution::merge_two_lists(l1.into(), l2.into());
    assert_eq!(actual, expected.into());
}
