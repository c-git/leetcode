//! Solution for https://leetcode.com/problems/flatten-nested-list-iterator
//! 341. Flatten Nested List Iterator

struct NestedIterator {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {
    fn new(nestedList: Vec<NestedInteger>) -> Self {
        todo!("Fill in body")
    }

    fn next(&self) -> i32 {
        todo!("Fill in body")
    }

    fn has_next(&self) -> bool {
        todo!("Fill in body")
    }
}

/**
 * Your NestedIterator object will be instantiated and called as such:
 * let obj = NestedIterator::new(nestedList);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */

// << ---------------- Code below here is only for local use ---------------- >>

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    use NestedInteger as ni;

    #[rstest]
    #[case(vec![ni::List(vec![ni::Int(1),ni::Int(1)]),ni::Int(2),ni::List(vec![ni::Int(1),ni::Int(1)])], vec![1,1,2,1,1])]
    #[case(vec![ni::Int(1),ni::List(vec![ni::Int(4),ni::List(vec![ni::Int(6)])])], vec![1,4,6])]
    fn case(#[case] nested_list: Vec<NestedInteger>, #[case] expected: Vec<i32>) {
        let mut obj = NestedIterator::new(nested_list);
        let mut actual = vec![];
        while obj.has_next() {
            actual.push(obj.next());
        }
        assert_eq!(actual, expected);
    }
}
