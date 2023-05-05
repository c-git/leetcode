// Patterned fix for weak links on https://github.com/timClicks/live-streams/blob/main/doubly_linked_list/src/weak_ref.rs

use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
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

impl Debug for NakedNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Node: count: {}, strings: {:?}, has_prev: {}, next: {:?}",
            self.count,
            self.strings,
            self.prev.is_some(),
            self.next.as_ref().map(|next| next.borrow())
        )
    }
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

    fn new_wrapped(count: usize, word: Word) -> Node {
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

    fn add_word(&mut self, word: &Word) {
        let is_newly_inserted = self.strings.insert(Rc::clone(word));
        debug_assert!(is_newly_inserted, "Doesn't cause a problem for the node but not expecting this to be called if it was already there");
    }

    fn remove_word(&mut self, word: &Word) {
        let is_already_existing = self.strings.remove(word);
        debug_assert!(is_already_existing, "Doesn't cause a problem for the node but not expecting this to be called if it was already there");
    }

    fn get_word(&self) -> String {
        self.strings
            .iter()
            .next()
            .expect("Empty nodes not supposed to be allowed")
            .to_string()
    }
}

#[derive(Default)]
struct LinkedList {
    head: LinkFwd,
    tail: LinkBack,
}

impl Debug for LinkedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LinkedList: has_tail: {}, head: {:?}",
            self.tail.is_some(),
            self.head.as_ref().map(|head| head.borrow())
        )
    }
}

impl LinkedList {
    fn insert_at_head(&mut self, word: Word) -> Node {
        if let Some(head) = self.head.as_mut() {
            debug_assert!(self.tail.is_some());
            if head.borrow().count == 1 {
                // Add to existing head
                head.borrow_mut().add_word(&word);
                Rc::clone(head)
            } else {
                // Push head forward and make a new head
                debug_assert!(head.borrow().prev.is_none());
                let node = NakedNode::new_wrapped(1, word);
                head.borrow_mut().prev = Some(Rc::downgrade(&node));
                node.borrow_mut().next = Some(Rc::clone(head));
                self.head = Some(Rc::clone(&node));
                node
            }
        } else {
            debug_assert!(self.tail.is_none());
            let node = NakedNode::new_wrapped(1, word);
            self.head = Some(Rc::clone(&node));
            self.tail = Some(Rc::downgrade(&node));
            node
        }
    }

    fn insert_next_to_node(&mut self, existing_node: Node, count: usize, word: Word) -> Node {
        let new_node = NakedNode::new_wrapped(count, word);

        if existing_node.borrow().next.is_some() {
            existing_node
                .borrow()
                .next
                .as_ref()
                .unwrap()
                .borrow_mut()
                .prev = Some(Rc::downgrade(&new_node));
            new_node.borrow_mut().next = existing_node.borrow().next_clone();
            existing_node.borrow_mut().next = Some(Rc::clone(&new_node));
            new_node.borrow_mut().prev = Some(Rc::downgrade(&existing_node));
        } else {
            debug_assert!(self
                .tail
                .as_ref()
                .unwrap()
                .ptr_eq(&Rc::downgrade(&existing_node)));
            existing_node.borrow_mut().next = Some(Rc::clone(&new_node));
            new_node.borrow_mut().prev = Some(Rc::downgrade(&existing_node));
            self.tail = Some(Rc::downgrade(&new_node));
        }
        new_node
    }

    fn increment_word(&mut self, word: &Word, node: Node) -> Node {
        debug_assert!(node.borrow().strings.contains(word));

        let is_single_word = node.borrow().strings.len() == 1;
        let should_update_next = matches!(
            node.borrow()
                .next
                .as_ref()
                .map(|next| next.borrow().count == node.borrow().count + 1),
            Some(true)
        );

        match (is_single_word, should_update_next) {
            (true, true) => {
                // Remove current node and move value to next node
                let next = node.borrow().next_clone().expect("Checked above");
                next.borrow_mut().add_word(word);
                self.remove(node);
                next
            }
            (true, false) => {
                node.borrow_mut().count += 1;
                node
            }
            (false, true) => {
                node.borrow_mut().remove_word(word);
                let next = node.borrow().next_clone().expect("Checked above");
                next.borrow_mut().add_word(word);
                next
            }
            (false, false) => {
                node.borrow_mut().remove_word(word);
                let new_count = node.borrow().count + 1;
                self.insert_next_to_node(Rc::clone(&node), new_count, Rc::clone(word))
            }
        }
    }

    fn remove(&mut self, node: Node) {
        if let Some(head) = &self.head {
            debug_assert!(head.borrow().prev.is_none());
            if head.as_ptr() == node.as_ptr() {
                self.head = node.borrow().next_clone()
            }
        } else {
            debug_assert!(false, "There must be a head");
        }

        if let Some(tail) = &self.tail {
            if tail
                .upgrade()
                .expect("Tail is Some but not pointing to a valid node")
                .as_ptr()
                == node.as_ptr()
            {
                self.tail = node.borrow().prev_clone();
            }
        } else {
            debug_assert!(false, "There must be a tail");
        }

        if let Some(prev_node) = &node.borrow().prev {
            prev_node
                .upgrade()
                .expect("Prev is some but not pointing to a valid node")
                .borrow_mut()
                .next = node.borrow().next_clone();
        }

        if let Some(next_node) = &node.borrow().next {
            next_node.borrow_mut().prev = node.borrow().prev_clone();
        }
    }

    fn decrement_word(&mut self, word: Word, node: Node) -> Option<Node> {
        debug_assert!(node.borrow().strings.contains(&word));

        let is_single_word = node.borrow().strings.len() == 1;
        let should_update_prev = matches!(
            node.borrow()
                .prev
                .as_ref()
                .map(|prev: &Weak<RefCell<NakedNode>>| {
                    prev.upgrade()
                        .expect("Prev is some but does not exist")
                        .borrow()
                        .count
                        == node.borrow().count - 1
                }),
            Some(true)
        );

        match (is_single_word, should_update_prev) {
            (true, true) => {
                // Remove current node and move value to prev node
                let prev = node
                    .borrow()
                    .prev_clone()
                    .expect("Checked above")
                    .upgrade()
                    .expect("Prev points to nothing");
                prev.borrow_mut().add_word(&word);
                self.remove(node);
                Some(prev)
            }
            (true, false) => {
                if node.borrow().count == 1 {
                    debug_assert_eq!(node.as_ptr(), self.head.as_ref().unwrap().as_ptr());
                    self.remove(node);
                    None
                } else {
                    node.borrow_mut().count -= 1;
                    Some(node)
                }
            }
            (false, true) => {
                node.borrow_mut().remove_word(&word);
                let prev = node
                    .borrow()
                    .prev_clone()
                    .expect("Checked above")
                    .upgrade()
                    .expect("Prev points to nothing");
                prev.borrow_mut().add_word(&word);
                Some(prev)
            }
            (false, false) => {
                node.borrow_mut().remove_word(&word);
                if node.borrow().count == 1 {
                    None
                } else {
                    Some(self.insert_prev_to_node(&node, node.borrow().count - 1, word))
                }
            }
        }
    }

    fn insert_prev_to_node(&mut self, node: &Node, count: usize, word: Word) -> Node {
        if let Some(prev) = node.borrow().prev.as_ref() {
            self.insert_next_to_node(prev.upgrade().expect("Prev point to nothing"), count, word)
        } else {
            let new_node = self.insert_at_head(word);
            new_node.borrow_mut().count = count;
            new_node
        }
    }

    fn count_num_words_as_str(&self) -> String {
        if self.head.is_some() {
            let mut result = String::new();
            let mut opt_node = Some(Rc::clone(self.head.as_ref().unwrap()));
            while let Some(node) = opt_node {
                result.push_str(&format!(
                    "{} ({}) -> ",
                    node.borrow().count,
                    node.borrow().strings.len(),
                ));
                opt_node = node.borrow().next_clone();
            }
            result
        } else {
            "No Nodes".to_string()
        }
    }
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
    pub fn new() -> Self {
        Default::default()
    }

    pub fn inc(&mut self, key: String) {
        let word = Rc::new(key);
        let new_node = match self.items_map.get(&word) {
            Some(node) => self.count_list.increment_word(&word, Rc::clone(node)),
            None => self.count_list.insert_at_head(Rc::clone(&word)),
        };
        self.items_map.insert(word, new_node);
    }

    pub fn dec(&mut self, key: String) {
        let word = Rc::new(key);
        let node = self
            .items_map
            .get(&word)
            .expect("Based on constraints in the question");

        if let Some(new_node) = self
            .count_list
            .decrement_word(Rc::clone(&word), Rc::clone(node))
        {
            self.items_map.insert(word, new_node);
        } else {
            self.items_map.remove(&word);
        }
    }

    pub fn get_max_key(&self) -> String {
        self.get_max_key_helper().unwrap_or_default()
    }

    pub fn get_min_key(&self) -> String {
        self.get_min_key_helper().unwrap_or_default()
    }

    fn get_max_key_helper(&self) -> Option<String> {
        self.count_list.tail.as_ref().map(|tail| {
            tail.upgrade()
                .expect("Tail points to nothing")
                .borrow()
                .get_word()
        })
    }

    fn get_min_key_helper(&self) -> Option<String> {
        self.count_list
            .head
            .as_ref()
            .map(|head| head.borrow().get_word())
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
        let mut all_one = AllOne::new();
        all_one.inc("hello".into());
        all_one.inc("hello".into());
        assert_eq!(all_one.get_max_key(), "hello"); // return "hello"
        assert_eq!(all_one.get_min_key(), "hello"); // return "hello"
        all_one.inc("leet".into());
        assert_eq!(all_one.get_max_key(), "hello"); // return "hello"
        assert_eq!(all_one.get_min_key(), "leet"); // return "leet"
    }

    #[test]
    fn case2() {
        let mut all_one = AllOne::new();
        all_one.inc("hello".into());
        all_one.inc("goodbye".into());
        all_one.inc("hello".into());
        all_one.inc("hello".into());
        all_one.get_max_key();
        all_one.inc("leet".into());
        all_one.inc("code".into());
        all_one.dec("leet".into());
        all_one.inc("hello".into());
        all_one.inc("leet".into());
        all_one.inc("code".into());
        all_one.inc("code".into());
        all_one.get_max_key();
    }

    #[test]
    fn case3() {
        let mut obj = AllOne::new();
        println!("{:?}", obj.count_list.count_num_words_as_str());
        dbg!(obj.inc("hello".into()));
        println!("{:?}", obj.count_list.count_num_words_as_str());
        dbg!(obj.inc("goodbye".into()));
        println!("{:?}", obj.count_list.count_num_words_as_str());
        dbg!(obj.inc("hello".into()));
        println!("{:?}", obj.count_list.count_num_words_as_str());
        dbg!(obj.inc("hello".into()));
        println!("{:?}", obj.count_list.count_num_words_as_str());
        assert_eq!(obj.get_max_key(), "hello");
        dbg!(obj.inc("leet".into()));
        println!("{:?}", obj.count_list.count_num_words_as_str());
        dbg!(obj.inc("code".into()));
        println!("{:?}", obj.count_list.count_num_words_as_str());
        dbg!(obj.inc("leet".into()));
        println!("{:?}", obj.count_list.count_num_words_as_str());
        dbg!(obj.dec("hello".into()));
        println!("{:?}", obj.count_list.count_num_words_as_str());
        dbg!(obj.inc("leet".into()));
        println!("{:?}", obj.count_list.count_num_words_as_str());
        dbg!(obj.inc("code".into()));
        println!("{:?}", obj.count_list.count_num_words_as_str());
        dbg!(obj.inc("code".into()));
        println!(
            // Print map values
            "{:?}",
            obj.items_map
                .iter()
                .map(|(k, v)| format!("{k}: {}", v.borrow().count))
                .collect::<Vec<String>>()
        );
        println!("{:?}", obj.count_list.count_num_words_as_str());
        assert!(["leet", "code"].contains(&&obj.get_max_key()[..]));
    }

    #[test]
    fn case4() {
        let mut obj = AllOne::new();
        dbg!(obj.inc("a".into()));
        println!("{:?}", obj.count_list.count_num_words_as_str());
        dbg!(obj.inc("b".into()));
        println!("{:?}", obj.count_list.count_num_words_as_str());
        dbg!(obj.inc("c".into()));
        println!("{:?}", obj.count_list.count_num_words_as_str());
        dbg!(obj.inc("d".into()));
        println!("{:?}", obj.count_list.count_num_words_as_str());
        dbg!(obj.inc("a".into()));
        println!("{:?}", obj.count_list.count_num_words_as_str());
        dbg!(obj.inc("b".into()));
        println!("{:?}", obj.count_list.count_num_words_as_str());
        dbg!(obj.inc("c".into()));
        println!("{:?}", obj.count_list.count_num_words_as_str());
        dbg!(obj.inc("d".into()));
        println!("{:?}", obj.count_list.count_num_words_as_str());
        dbg!(obj.inc("c".into()));
        println!("{:?}", obj.count_list.count_num_words_as_str());
        dbg!(obj.inc("d".into()));
        println!("{:?}", obj.count_list.count_num_words_as_str());
        dbg!(obj.inc("d".into()));
        println!("{:?}", obj.count_list.count_num_words_as_str());
        dbg!(obj.inc("a".into()));
        println!("{:?}", obj.count_list.count_num_words_as_str());
        assert_eq!(obj.get_min_key(), "b");
    }

    #[test]
    fn case5() {
        let mut obj = AllOne::new();
        obj.inc("hello".into());
        obj.inc("l".into());
        obj.inc("l".into());
        obj.inc("l".into());
        obj.inc("k".into());
        obj.inc("k".into());
        obj.inc("k".into());
        obj.inc("j".into());
        obj.inc("j".into());
        obj.inc("j".into());
        obj.dec("j".into());
        obj.dec("k".into());
        obj.get_max_key();
    }
}
