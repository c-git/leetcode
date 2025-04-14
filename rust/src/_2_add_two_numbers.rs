//! Solution for https://leetcode.com/problems/add-two-numbers
//! 2. Add Two Numbers

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
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut has_carry = false;
        let mut result = None;
        let mut tail = &mut result;
        loop {
            (l1, l2, has_carry) = match (l1, l2, has_carry) {
                (None, None, true) => {
                    *tail = Some(Box::new(ListNode::new(1)));
                    (None, None, false)
                }
                (None, None, false) => break,
                (None, Some(x), has_carry) | (Some(x), None, has_carry) => {
                    let next_val = x.val + if has_carry { 1 } else { 0 };
                    *tail = Some(Box::new(ListNode::new(next_val % 10)));
                    tail = &mut tail.as_mut().unwrap().next;
                    (x.next, None, next_val >= 10)
                }
                (Some(x), Some(y), has_carry) => {
                    let next_val = x.val + y.val + if has_carry { 1 } else { 0 };
                    *tail = Some(Box::new(ListNode::new(next_val % 10)));
                    tail = &mut tail.as_mut().unwrap().next;
                    (x.next, y.next, next_val >= 10)
                }
            }
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
    #[case(ListHead::from(vec![2,4,3]).into(), ListHead::from(vec![5,6,4]).into(), ListHead::from(vec![7,0,8]).into())]
    #[case(ListHead::from(vec![0]).into(), ListHead::from(vec![0]).into(), ListHead::from(vec![0]).into())]
    #[case(ListHead::from(vec![9,9,9,9,9,9,9]).into(), ListHead::from(vec![9,9,9,9]).into(), ListHead::from(vec![8,9,9,9,0,0,0,1]).into())]
    fn case(
        #[case] l1: Option<Box<ListNode>>,
        #[case] l2: Option<Box<ListNode>>,
        #[case] expected: Option<Box<ListNode>>,
    ) {
        let actual = Solution::add_two_numbers(l1, l2);
        assert_eq!(actual, expected);
    }
}
