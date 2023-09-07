//! Solution for https://leetcode.com/problems/reverse-linked-list-ii
//! 92. Reverse Linked List II

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
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        // Taken from https://leetcode.com/problems/reverse-linked-list-ii/solutions/4011862/92-40-two-pointers-stack-recursion/
        println!("At start: {head:?} left: {left} right: {right}");
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut before = &mut dummy;
        for i in 1..left {
            println!("i={i} before = {before:?}");
            before = &mut before.as_mut()?.next;
        }
        println!("After 1st loop. before: {before:?}");

        let mut node = before.as_mut()?.next.take();
        let mut node2 = node.as_mut()?.next.take();
        println!("node: {node:?} node2: {node2:?}");
        for _ in left..right {
            let node3 = node2.as_mut()?.next.take();
            println!("node: {node:?} node2: {node2:?} node3: {node3:?}");
            node2.as_mut()?.next = node;
            node = node2;
            node2 = node3;
        }
        println!("After 2nd loop. node: {node:?} node2: {node2:?}");

        let mut rev_tail = &mut node;
        for _ in left..right {
            println!("rev_tail: {rev_tail:?}");
            rev_tail = &mut rev_tail.as_mut()?.next;
        }
        println!("After 3rd loop. rev_tail: {rev_tail:?} node2: {node2:?} ");
        rev_tail.as_mut()?.next = node2;
        before.as_mut()?.next = node;

        println!("dummy: {dummy:?}");
        dummy.unwrap().next
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
    #[case(ListHead::from(vec![1,2,3,4,5]).into(), 2, 4, ListHead::from(vec![1,4,3,2,5]).into())]
    #[case(ListHead::from(vec![5]).into(), 1, 1, ListHead::from(vec![5]).into())]
    #[case(ListHead::from(vec![1,2,3,4,5]).into(), 1, 4, ListHead::from(vec![4,3,2,1,5]).into())]
    #[case(ListHead::from(vec![1,2,3,4,5]).into(), 1, 5, ListHead::from(vec![5,4,3,2,1]).into())]
    #[case(ListHead::from(vec![1,2,3,4,5]).into(), 3, 5, ListHead::from(vec![1,2,5,4,3]).into())]
    fn case(
        #[case] head: Option<Box<ListNode>>,
        #[case] left: i32,
        #[case] right: i32,
        #[case] expected: Option<Box<ListNode>>,
    ) {
        let actual = Solution::reverse_between(head, left, right);
        assert_eq!(actual, expected);
    }
}
