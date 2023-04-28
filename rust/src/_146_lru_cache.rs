use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

type Node = Rc<RefCell<NakedNode>>;

#[derive(PartialEq, Eq)]
struct NakedNode {
    key: i32, // Needs to store key to ensure they are unique
    value: i32,
    prev: Option<Node>,
    next: Option<Node>,
}

impl NakedNode {
    fn new(key: i32, value: i32) -> Self {
        Self {
            key,
            value,
            // TODO change to use WeakRefs for the back links to prevent memory leak
            prev: Default::default(),
            next: Default::default(),
        }
    }

    fn new_wrapped(key: i32, value: i32) -> Node {
        Rc::new(RefCell::new(Self::new(key, value)))
    }
}

impl Debug for NakedNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prev_status = if self.prev.is_some() {
            "is_some"
        } else {
            "is_none"
        };
        f.debug_struct("NakedNode")
            .field("key", &self.key)
            .field("value", &self.value)
            .field("prev", &prev_status)
            .field("next", &self.next)
            .finish()
    }
}

#[derive(Default, Debug)]
struct LinkedList {
    head: Option<Node>,
    // TODO change to use WeakRefs for the back links to prevent memory leak
    tail: Option<Node>,
}

impl LinkedList {
    fn insert(&mut self, node: Node) {
        // Update head prev to new node
        if let Some(head) = &self.head {
            head.borrow_mut().prev = Some(Rc::clone(&node));
        }

        // Set new node next to current head
        node.borrow_mut().next = self.head.as_ref().map(Rc::clone);

        // Update head to new node
        self.head = Some(Rc::clone(&node));

        // If tail not set then this new node is the tail
        if self.tail.is_none() {
            self.tail = Some(node);
        }
    }

    fn remove(&mut self, node: Node) {
        if let Some(head) = &self.head {
            if head == &node {
                self.head = node.borrow().next.as_ref().map(Rc::clone);
            }
        }

        if let Some(tail) = &self.tail {
            if tail == &node {
                self.tail = node.borrow().prev.as_ref().map(Rc::clone);
            }
        }

        if let Some(prev_node) = &node.borrow().prev {
            prev_node.borrow_mut().next = node.borrow().next.as_ref().map(Rc::clone);
        }

        if let Some(next_node) = &node.borrow().next {
            next_node.borrow_mut().prev = node.borrow().prev.as_ref().map(Rc::clone);
        }
    }

    fn get_last(&self) -> Option<Node> {
        self.tail.as_ref().map(Rc::clone)
    }
}

#[derive(Debug)]
struct LRUCache {
    capacity: usize,
    size: usize, // Current number of items in the Cache
    items_list: LinkedList,
    items_map: HashMap<i32, Node>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        // Constraints ensure that capacity is between 1 and 3000
        debug_assert!(
            capacity > 0,
            "Assumptions made in insert do not hold if capacity is 0"
        );
        let capacity = capacity as usize;
        Self {
            capacity,
            size: 0,
            items_list: Default::default(),
            items_map: Default::default(),
        }
    }

    /// Removes the node from the cache if it exists and returns it
    /// The item will no longer be in the hashmap nor the linked list
    fn extract_node(&mut self, key: i32) -> Option<Node> {
        let result = self.items_map.remove(&key)?;

        self.items_list.remove(Rc::clone(&result));

        Some(result)
    }

    fn get_helper(&mut self, key: i32) -> Option<i32> {
        let node = self.extract_node(key)?;

        // Get value to be returned
        let result = node.borrow().value;

        // Insert the item back into the cache to update use
        self.insert_node(node);

        Some(result)
    }

    pub fn get(&mut self, key: i32) -> i32 {
        self.get_helper(key).unwrap_or(-1)
    }

    pub fn put(&mut self, key: i32, value: i32) {
        // Create node
        let node: Node = NakedNode::new_wrapped(key, value);

        // Remove any node that already exists with the key
        let old_node = self.extract_node(key);
        let is_replacing_value = old_node.is_some();

        // Insert Node
        self.insert_node(node);

        // Increase size or kick out an element if not replacing a value
        if !is_replacing_value {
            let capacity = self.capacity;
            match self.size {
                val if val < capacity => {
                    self.size += 1;
                }
                val if val == capacity => {
                    self.remove_lru();
                }
                _ => unreachable!("Size should never exceed capacity"),
            }
        }
    }

    fn insert_node(&mut self, node: Node) {
        self.items_map.insert(node.borrow().key, Rc::clone(&node));
        self.items_list.insert(node);
    }

    fn remove_lru(&mut self) {
        debug_assert!(self.size > 0);
        let last_node = self
            .items_list
            .get_last()
            .expect("Expected the list to have at least 1 item");
        self.extract_node(last_node.borrow().key);
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut lru_cache = LRUCache::new(2);
        lru_cache.put(1, 1); // cache is {1=1}
        lru_cache.put(2, 2); // cache is {1=1, 2=2}
        assert_eq!(lru_cache.get(1), 1); // return 1
        lru_cache.put(3, 3); // LRU key was 2, evicts key 2, cache is {1=1, 3=3}
        assert_eq!(lru_cache.get(2), -1); // returns -1 (not found)
        lru_cache.put(4, 4); // LRU key was 1, evicts key 1, cache is {4=4, 3=3}
        assert_eq!(lru_cache.get(1), -1); // return -1 (not found)
        assert_eq!(lru_cache.get(3), 3); // return 3
        assert_eq!(lru_cache.get(4), 4); // return 4
    }
}
