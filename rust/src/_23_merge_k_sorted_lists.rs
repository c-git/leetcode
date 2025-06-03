//! Solution for https://leetcode.com/problems/merge-k-sorted-lists
//! 23. Merge k Sorted Lists

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

struct HeapItem(Box<ListNode>);

impl PartialEq for HeapItem{
    fn eq(&self, other: &Self) -> bool {
        // Doesn't do a deep check for equality
        self.0.val == other.0.val
    }
}

impl Eq for HeapItem{}

impl  PartialOrd for HeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for HeapItem{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Provide a reversed result so min will come to the top of the heap
        other.0.val.cmp(&self.0.val)
    }
}
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut result = None;
        let mut tail = &mut result;
        let mut heap = std::collections::BinaryHeap::new();
        for list_head in lists.into_iter().flatten(){
                heap.push(HeapItem(list_head));
        }
        while let Some(HeapItem(mut list_head)) = heap.pop(){
            if let Some(next) = list_head.next.take(){
             heap.push(HeapItem(next));   
            }
            *tail = Some(list_head);
            tail = &mut tail.as_mut().unwrap().next;
        }
        result
    }
}


// << ---------------- Code below here is only for local use ---------------- >>


use cargo_leet::ListNode;
pub struct Solution;

#[cfg(test)]
mod tests {
    use cargo_leet::ListHead;

    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(
        vec![
                ListHead::from(vec![1,4,5]).into(),
                ListHead::from(vec![1,3,4]).into(),
                ListHead::from(vec![2,6]).into()
            ], 
        ListHead::from(vec![1,1,2,3,4,4,5,6]).into())]
    #[case(vec![], ListHead::from(vec![]).into())]
    #[case(vec![ListHead::from(vec![]).into()], ListHead::from(vec![]).into())]
    fn case(#[case] lists: Vec<Option<Box<ListNode>>>, #[case] expected: Option<Box<ListNode>>) {
        let actual = Solution::merge_k_lists(lists);
        assert_eq!(actual, expected);
    }
}
