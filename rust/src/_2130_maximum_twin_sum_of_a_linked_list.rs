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
    pub fn pair_sum(mut head: Option<Box<ListNode>>) -> i32 {
        // Replaced O(1) memory option with a "faster" option that uses O(1) memory but with a much bigger constant
        let mut result = 0; // Initialized to 0 because, values are all positive and min of 2
        let mut values = Vec::with_capacity(100_000); // Size based on constraint in question

        // Store values in vec
        while let Some(node) = head {
            values.push(node.val);
            head = node.next;
        }

        // Walk pairs and compare sums
        let n = values.len();
        for i in 0..n / 2 {
            let sum = values[i] + values[n - 1 - i];
            if sum > result {
                result = sum;
            }
        }
        result
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
        let input: ListHead = vec![5, 4, 2, 1].into();
        let expected = 6;
        let actual = Solution::pair_sum(input.into());
        assert_eq!(actual, expected);
    }

    #[test]
    fn case2() {
        let input: ListHead = vec![4, 2, 2, 3].into();
        let expected = 7;
        let actual = Solution::pair_sum(input.into());
        assert_eq!(actual, expected);
    }

    #[test]
    fn case3() {
        let input: ListHead = vec![1, 100000].into();
        let expected = 100001;
        let actual = Solution::pair_sum(input.into());
        assert_eq!(actual, expected);
    }
}
