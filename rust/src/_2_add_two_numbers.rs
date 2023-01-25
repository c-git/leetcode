use crate::helper::ListNode;

struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        todo!()
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
