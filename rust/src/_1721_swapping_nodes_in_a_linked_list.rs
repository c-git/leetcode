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
    pub fn swap_nodes(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let k = k as usize;
        let mut fast = &head;
        let mut slow = head.as_ref().expect("Constraint is n >= 1");

        // Move fast ahead of slow by k
        for _ in 0..k {
            fast = &fast
                .as_ref()
                .expect("Based on constraint in question k <= n")
                .next;
        }

        // Move slow to k before the end
        let mut second_index = 0; // TODO switch to 1 based indexing
        while let Some(node) = fast {
            fast = &node.next;
            slow = slow
                .next
                .as_ref()
                .expect("This is behind fast so must exist");
            second_index += 1;
        }

        // Save value from 2nd node to update the first
        let second_value = slow.val;

        // Walk the list again and update the nodes
        let mut node = &mut head;
        for _ in 0..k - 1 {
            node = &mut node.as_mut().expect("k <= n by constraint").next;
        }
        let first_value = node.as_ref().expect("At first node").val;
        node.as_mut().expect("At first node").val = second_value;
        for _ in k - 1..second_index {
            node = &mut node.as_mut().expect("k <= n by constraint").next;
        }
        node.as_mut().expect("At second node").val = first_value;

        head
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
        let head: ListHead = vec![1, 2, 3, 4, 5].into();
        let k = 2;
        let expected: ListHead = vec![1, 4, 3, 2, 5].into();
        let actual = Solution::swap_nodes(head.into(), k);
        assert_eq!(actual, expected.into());
    }

    #[test]
    fn case2() {
        let head: ListHead = vec![7, 9, 6, 6, 7, 8, 3, 0, 9, 5].into();
        let k = 5;
        let expected: ListHead = vec![7, 9, 6, 6, 8, 7, 3, 0, 9, 5].into();
        let actual = Solution::swap_nodes(head.into(), k);
        assert_eq!(actual, expected.into());
    }

    #[test]
    fn case3() {
        let head: ListHead = vec![5].into();
        let k = 1;
        let expected: ListHead = vec![5].into();
        let actual = Solution::swap_nodes(head.into(), k);
        assert_eq!(actual, expected.into());
    }

    #[test]
    fn case4() {
        let head: ListHead = vec![1, 2, 3].into();
        let k = 1;
        let expected: ListHead = vec![3, 2, 1].into();
        let actual = Solution::swap_nodes(head.into(), k);
        assert_eq!(actual, expected.into());
    }

    #[test]
    fn case5() {
        let head: ListHead = vec![1, 2].into();
        let k = 1;
        let expected: ListHead = vec![2, 1].into();
        let actual = Solution::swap_nodes(head.into(), k);
        assert_eq!(actual, expected.into());
    }

    #[test]
    fn case6() {
        let head: ListHead = vec![1, 2].into();
        let k = 2;
        let expected: ListHead = vec![2, 1].into();
        let actual = Solution::swap_nodes(head.into(), k);
        assert_eq!(actual, expected.into());
    }
}
