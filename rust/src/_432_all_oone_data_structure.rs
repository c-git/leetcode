// Patterned fix for weak links on https://github.com/timClicks/live-streams/blob/main/doubly_linked_list/src/weak_ref.rs

use std::collections::{HashMap, HashSet};
use std::iter::once;
use std::rc::Weak;
use std::{cell::RefCell, rc::Rc};

type Node = Rc<RefCell<NakedNode>>;
type NodeWeak = Weak<RefCell<NakedNode>>;
type LinkFwd = Option<Node>; // Used only for forward links
type LinkBack = Option<NodeWeak>;
type Word = Rc<String>;

struct NakedNode {
    count: usize, // The count for the strings here
    strings: HashSet<Word>,
    prev: LinkBack,
    next: LinkFwd,
}

impl NakedNode {
    fn new(count: usize, word: Word) -> Self {
        Self {
            count,
            strings: once(word).collect(),
            prev: Default::default(),
            next: Default::default(),
        }
    }

    fn new_node(count: usize, word: Word) -> Node {
        Rc::new(RefCell::new(Self::new(count, word)))
    }

    fn prev_clone(&self) -> LinkBack {
        self.prev
            .as_ref()
            .map(|x| Rc::downgrade(&x.upgrade().expect("Invalid link to node in a Some")))
    }

    fn next_clone(&self) -> LinkFwd {
        self.next.as_ref().map(Rc::clone)
    }
}

#[derive(Default)]
struct LinkedList {
    head: LinkFwd,
    tail: LinkBack,
}

#[derive(Default)]
struct AllOne {
    /// Stores the list of nodes that have a count and the words with that count
    count_list: LinkedList,

    /// Stores a mapping from a word to the node in the list that has that word in it
    items_map: HashMap<Word, Node>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AllOne {
    fn new() -> Self {
        Default::default()
    }

    fn inc(&mut self, key: String) {
        todo!()
    }

    fn dec(&mut self, key: String) {
        todo!()
    }

    fn get_max_key(&self) -> String {
        todo!()
    }

    fn get_min_key(&self) -> String {
        todo!()
    }
}

/**
 * Your AllOne object will be instantiated and called as such:
 * let obj = AllOne::new();
 * obj.inc(key);
 * obj.dec(key);
 * let ret_3: String = obj.get_max_key();
 * let ret_4: String = obj.get_min_key();
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut allOne = AllOne::new();
        allOne.inc("hello".into());
        allOne.inc("hello".into());
        assert_eq!(allOne.get_max_key(), "hello"); // return "hello"
        assert_eq!(allOne.get_min_key(), "hello"); // return "hello"
        allOne.inc("leet".into());
        assert_eq!(allOne.get_max_key(), "hello"); // return "hello"
        assert_eq!(allOne.get_min_key(), "leet"); // return "leet"
    }
}
