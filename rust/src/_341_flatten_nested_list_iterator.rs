//! Solution for https://leetcode.com/problems/flatten-nested-list-iterator
//! 341. Flatten Nested List Iterator

pub struct NestedIterator {
    list: Vec<i32>,
    index: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {
    pub fn new(nested_list: Vec<NestedInteger>) -> Self {
        let mut list = vec![];
        Self::get_values(nested_list, &mut list);
        Self { list, index: 0 }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> i32 {
        let result = self.list[self.index];
        self.index += 1;
        result
    }

    fn get_values(nested_list: Vec<NestedInteger>, result: &mut Vec<i32>) {
        let mut pending: Vec<Vec<usize>> = nested_list
            .iter()
            .enumerate()
            .rev()
            .map(|(i, _)| vec![i])
            .collect();
        while let Some(mut path) = pending.pop() {
            debug_assert!(
                !path.is_empty(),
                "There should not be empty vectors in the pending list"
            );

            let mut element = &nested_list[path[0]];

            for (path_idx, &list_idx) in path.iter().enumerate().skip(1) {
                let is_last = path_idx == path.len() - 1;
                element = match element {
                    NestedInteger::Int(_) => {
                        unreachable!("Everything but the last must be a List")
                    }
                    NestedInteger::List(inner) => {
                        if is_last && list_idx < inner.len() - 1 {
                            // Queue up next item to be checked
                            let mut new_path = path.clone();
                            *new_path.last_mut().unwrap() += 1;
                            pending.push(new_path);
                        }

                        // Return next item in path
                        &inner[list_idx]
                    }
                }
            }

            match element {
                NestedInteger::Int(val) => result.push(*val),
                NestedInteger::List(inner) => {
                    if !inner.is_empty() {
                        path.push(0);
                        pending.push(path);
                    }
                }
            }
        }
    }

    pub fn has_next(&self) -> bool {
        self.index < self.list.len()
    }
}

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
    #[case(vec![ni::List(vec![])], vec![])]
    #[case(vec![ni::Int(1),ni::List(vec![ni::Int(4),ni::List(vec![ni::Int(6)])]),ni::List(vec![])], vec![1,4,6])]
    fn case(#[case] nested_list: Vec<NestedInteger>, #[case] expected: Vec<i32>) {
        let mut obj = NestedIterator::new(nested_list);
        let mut actual = vec![];
        while obj.has_next() {
            actual.push(obj.next());
        }
        assert_eq!(actual, expected);
    }
}
