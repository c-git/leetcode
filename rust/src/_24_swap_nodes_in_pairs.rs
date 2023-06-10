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
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut left = &mut head;
        while left.is_some() {
            let mut right = left.as_mut().unwrap().next.take();
            if right.is_some() {
                left.as_mut().unwrap().next = right.as_mut().unwrap().next.take();
                let temp = left.take();
                *left = right;
                left.as_mut().unwrap().next = temp;
                left = &mut left.as_mut().unwrap().next.as_mut().unwrap().next;
            } else {
                break;
            }
        }
        head
    }
}

use cargo_leet::ListNode;

struct Solution;
#[cfg(test)]
mod tests {
    use cargo_leet::ListHead;

    use super::*;

    #[test]
    fn case1() {
        let input: ListHead = vec![1, 2, 3, 4].into();
        let expected: ListHead = vec![2, 1, 4, 3].into();
        let actual = Solution::swap_pairs(input.into());
        assert_eq!(actual, expected.into());
    }

    #[test]
    fn case2() {
        let input: ListHead = vec![].into();
        let expected: ListHead = vec![].into();
        let actual = Solution::swap_pairs(input.into());
        assert_eq!(actual, expected.into());
    }

    #[test]
    fn case3() {
        let input: ListHead = vec![1].into();
        let expected: ListHead = vec![1].into();
        let actual = Solution::swap_pairs(input.into());
        assert_eq!(actual, expected.into());
    }

    #[test]
    fn case4() {
        let input: ListHead = vec![1, 2, 3, 4, 5].into();
        let expected: ListHead = vec![2, 1, 4, 3, 5].into();
        let actual = Solution::swap_pairs(input.into());
        assert_eq!(actual, expected.into());
    }
}
