//! Solution for https://leetcode.com/problems/flatten-nested-list-iterator
//! 341. Flatten Nested List Iterator

pub struct NestedIterator {
    list: Vec<NestedInteger>,
    /// Each element stores a path into `list`. All numbers except last are guaranteed to be [`NestedList:List`]`
    ///
    /// eg. vec![0,2] says go into the List at index 0 and look at the item at index 2
    pending: Vec<Vec<usize>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {
    pub fn new(nested_list: Vec<NestedInteger>) -> Self {
        let pending = if nested_list.is_empty() {
            vec![]
        } else {
            vec![vec![0]]
        };
        Self {
            list: nested_list,
            pending,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> i32 {
        debug_assert!(
            !self.pending.is_empty(),
            "Should return a result or option but since it doesn't this is a precondition"
        );

        let mut path = self
            .pending
            .pop()
            .expect("Expected at least one element to be pending");

        debug_assert!(
            !path.is_empty(),
            "There should not be empty vectors in the pending list"
        );

        let mut element = &self.list[path[0]];

        if path.len() == 1 && path[0] < self.list.len() - 1 {
            // Dealing with top level list
            self.pending.push(vec![path[0] + 1]);
        } else {
            for (path_idx, &list_idx) in path.iter().enumerate().skip(1) {
                let is_last = path_idx == path.len() - 1;
                element = match element {
                    NestedInteger::Int(_) => {
                        unreachable!("Everything but the last should be a List")
                    }
                    NestedInteger::List(inner) => {
                        if is_last && list_idx < inner.len() - 1 {
                            // Queue up next item to be checked
                            let mut new_path = path.clone();
                            *new_path.last_mut().unwrap() += 1;
                            self.pending.push(new_path);
                        }

                        // Return next item in path
                        &inner[list_idx]
                    }
                }
            }
        }

        match element {
            NestedInteger::Int(val) => *val,
            NestedInteger::List(inner) => {
                if !inner.is_empty() {
                    path.push(0);
                    self.pending.push(path);
                    self.next()
                } else {
                    unreachable!("Constraint said there would be no empty Lists")
                }
            }
        }
    }

    pub fn has_next(&self) -> bool {
        !self.pending.is_empty()
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
