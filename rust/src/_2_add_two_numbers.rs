use crate::helper::ListNode;

struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // Source: https://github.com/zwhitchcox/leetcode/blob/master/src/0002_add_two_numbers.rs
        let mut result = None;

        // Make inputs mutable to be able to walk down the list
        let mut l1 = l1;
        let mut l2 = l2;

        let mut carry = 0;
        let mut curr = &mut result;
        while l1.is_some() || l2.is_some() || carry != 0 {
            let mut sum = carry;
            if let Some(node) = l1 {
                sum += node.val;
                l1 = node.next;
            }
            if let Some(node) = l2 {
                sum += node.val;
                l2 = node.next;
            }
            carry = sum / 10;
            *curr = Some(Box::new(ListNode::new(sum % 10)));
            curr = &mut curr.as_mut().unwrap().next;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use crate::helper::ListHead;

    use super::Solution;

    #[test]
    fn case1() {
        let l1: ListHead = vec![2, 4, 3].into();
        let l2: ListHead = vec![5, 6, 4].into();
        let expected: ListHead = vec![7, 0, 8].into();

        let actual = Solution::add_two_numbers(l1.into(), l2.into());
        assert_eq!(expected, actual.into());
    }

    #[test]
    fn case2() {
        let l1: ListHead = vec![0].into();
        let l2: ListHead = vec![0].into();
        let expected: ListHead = vec![0].into();

        let actual = Solution::add_two_numbers(l1.into(), l2.into());
        assert_eq!(expected, actual.into());
    }

    #[test]
    fn case3() {
        let l1: ListHead = vec![9, 9, 9, 9, 9, 9, 9].into();
        let l2: ListHead = vec![9, 9, 9, 9].into();
        let expected: ListHead = vec![8, 9, 9, 9, 0, 0, 0, 1].into();

        let actual = Solution::add_two_numbers(l1.into(), l2.into());
        assert_eq!(expected, actual.into());
    }
}
