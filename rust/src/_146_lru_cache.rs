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

    #[test]
    fn case2() {
        let mut obj = LRUCache::new(105);
        obj.put(33, 219);
        obj.get(39);
        obj.put(96, 56);
        obj.get(129);
        obj.get(115);
        obj.get(112);
        obj.put(3, 280);
        obj.get(40);
        obj.put(85, 193);
        obj.put(10, 10);
        obj.put(100, 136);
        obj.put(12, 66);
        obj.put(81, 261);
        obj.put(33, 58);
        obj.get(3);
        obj.put(121, 308);
        obj.put(129, 263);
        obj.get(105);
        obj.put(104, 38);
        obj.put(65, 85);
        obj.put(3, 141);
        obj.put(29, 30);
        obj.put(80, 191);
        obj.put(52, 191);
        obj.put(8, 300);
        obj.get(136);
        obj.put(48, 261);
        obj.put(3, 193);
        obj.put(133, 193);
        obj.put(60, 183);
        obj.put(128, 148);
        obj.put(52, 176);
        obj.get(48);
        obj.put(48, 119);
        obj.put(10, 241);
        obj.get(124);
        obj.put(130, 127);
        obj.get(61);
        obj.put(124, 27);
        obj.get(94);
        obj.put(29, 304);
        obj.put(102, 314);
        obj.get(110);
        obj.put(23, 49);
        obj.put(134, 12);
        obj.put(55, 90);
        obj.get(14);
        obj.get(104);
        obj.put(77, 165);
        obj.put(60, 160);
        obj.get(117);
        obj.put(58, 30);
        obj.get(54);
        obj.get(136);
        obj.get(128);
        obj.get(131);
        obj.put(48, 114);
        obj.get(136);
        obj.put(46, 51);
        obj.put(129, 291);
        obj.put(96, 207);
        obj.get(131);
        obj.put(89, 153);
        obj.put(120, 154);
        obj.get(111);
        obj.get(47);
        obj.get(5);
        obj.put(114, 157);
        obj.put(57, 82);
        obj.put(113, 106);
        obj.put(74, 208);
        obj.get(56);
        obj.get(59);
        obj.get(100);
        obj.get(132);
        obj.put(127, 202);
        obj.get(75);
        obj.put(102, 147);
        obj.get(37);
        obj.put(53, 79);
        obj.put(119, 220);
        obj.get(47);
        obj.get(101);
        obj.get(89);
        obj.get(20);
        obj.get(93);
        obj.get(7);
        obj.put(48, 109);
        obj.put(71, 146);
        obj.get(43);
        obj.get(122);
        obj.put(3, 160);
        obj.get(17);
        obj.put(80, 22);
        obj.put(80, 272);
        obj.get(75);
        obj.get(117);
        obj.put(76, 204);
        obj.put(74, 141);
        obj.put(107, 93);
        obj.put(34, 280);
        obj.put(31, 94);
        obj.get(132);
        obj.put(71, 258);
        obj.get(61);
        obj.get(60);
        obj.put(69, 272);
        obj.get(46);
        obj.put(42, 264);
        obj.put(87, 126);
        obj.put(107, 236);
        obj.put(131, 218);
        obj.get(79);
        obj.put(41, 71);
        obj.put(94, 111);
        obj.put(19, 124);
        obj.put(52, 70);
        obj.get(131);
        obj.get(103);
        obj.get(81);
        obj.get(126);
        obj.put(61, 279);
        obj.put(37, 100);
        obj.get(95);
        obj.get(54);
        obj.put(59, 136);
        obj.put(101, 219);
        obj.put(15, 248);
        obj.put(37, 91);
        obj.put(11, 174);
        obj.put(99, 65);
        obj.put(105, 249);
        obj.get(85);
        obj.put(108, 287);
        obj.put(96, 4);
        obj.get(70);
        obj.get(24);
        obj.put(52, 206);
        obj.put(59, 306);
        obj.put(18, 296);
        obj.put(79, 95);
        obj.put(50, 131);
        obj.put(3, 161);
        obj.put(2, 229);
        obj.put(39, 183);
        obj.put(90, 225);
        obj.put(75, 23);
        obj.put(136, 280);
        obj.get(119);
        obj.put(81, 272);
        obj.get(106);
        obj.get(106);
        obj.get(70);
        obj.put(73, 60);
        obj.put(19, 250);
        obj.put(82, 291);
        obj.put(117, 53);
        obj.put(16, 176);
        obj.get(40);
        obj.put(7, 70);
        obj.put(135, 212);
        obj.get(59);
        obj.put(81, 201);
        obj.put(75, 305);
        obj.get(101);
        obj.put(8, 250);
        obj.get(38);
        obj.put(28, 220);
        obj.get(21);
        obj.put(105, 266);
        obj.get(105);
        obj.get(85);
        obj.get(55);
        obj.get(6);
        obj.put(78, 83);
        obj.get(126);
        obj.get(102);
        obj.get(66);
        obj.put(61, 42);
        obj.put(127, 35);
        obj.put(117, 105);
        obj.get(128);
        obj.get(102);
        obj.get(50);
        obj.put(24, 133);
        obj.put(40, 178);
        obj.put(78, 157);
        obj.put(71, 22);
        obj.get(25);
        obj.get(82);
        obj.get(129);
        obj.put(126, 12);
        obj.get(45);
        obj.get(40);
        obj.get(86);
        obj.get(100);
        obj.put(30, 110);
        obj.get(49);
        obj.put(47, 185);
        obj.put(123, 101);
        obj.get(102);
        obj.get(5);
        obj.put(40, 267);
        obj.put(48, 155);
        obj.get(108);
        obj.get(45);
        obj.put(14, 182);
        obj.put(20, 117);
        obj.put(43, 124);
        obj.get(38);
        obj.put(77, 158);
        obj.get(111);
        obj.get(39);
        obj.put(69, 126);
        obj.put(113, 199);
        obj.put(21, 216);
        obj.get(11);
        obj.put(117, 207);
        obj.get(30);
        obj.put(97, 84);
        obj.get(109);
        obj.put(99, 218);
        obj.get(109);
        obj.put(113, 1);
        obj.get(62);
        obj.put(49, 89);
        obj.put(53, 311);
        obj.get(126);
        obj.put(32, 153);
        obj.put(14, 296);
        obj.get(22);
        obj.put(14, 225);
        obj.get(49);
        obj.get(75);
        obj.put(61, 241);
        obj.get(7);
        obj.get(6);
        obj.get(31);
        obj.put(75, 15);
        obj.get(115);
        obj.put(84, 181);
        obj.put(125, 111);
        obj.put(105, 94);
        obj.put(48, 294);
        obj.get(106);
        obj.get(61);
        obj.put(53, 190);
        obj.get(16);
        obj.put(12, 252);
        obj.get(28);
        obj.put(111, 122);
        obj.get(122);
        obj.put(10, 21);
        obj.get(59);
        obj.get(72);
        obj.get(39);
        obj.get(6);
        obj.get(126);
        obj.put(131, 177);
        obj.put(105, 253);
        obj.get(26);
        obj.put(43, 311);
        obj.get(79);
        obj.put(91, 32);
        obj.put(7, 141);
        obj.get(38);
        obj.get(13);
        obj.put(79, 135);
        obj.get(43);
        obj.get(94);
        obj.put(80, 182);
        obj.get(53);
        obj.put(120, 309);
        obj.put(3, 109);
        obj.get(97);
        obj.put(9, 128);
        obj.put(114, 121);
        obj.get(56);
        obj.get(56);
        obj.put(124, 86);
        obj.put(34, 145);
        obj.get(131);
        obj.get(78);
        obj.put(86, 21);
        obj.get(98);
        obj.put(115, 164);
        obj.put(47, 225);
        obj.get(95);
        obj.put(89, 55);
        obj.put(26, 134);
        obj.put(8, 15);
        obj.get(11);
        obj.put(84, 276);
        obj.put(81, 67);
        obj.get(46);
        obj.get(39);
        obj.get(92);
        obj.get(96);
        obj.put(89, 51);
        obj.put(136, 240);
        obj.get(45);
        obj.get(27);
        obj.put(24, 209);
        obj.put(82, 145);
        obj.get(10);
        obj.put(104, 225);
        obj.put(120, 203);
        obj.put(121, 108);
        obj.put(11, 47);
        obj.get(89);
        obj.put(80, 66);
        obj.get(16);
        obj.put(95, 101);
        obj.get(49);
        obj.get(1);
        obj.put(77, 184);
        obj.get(27);
        obj.put(74, 313);
        obj.put(14, 118);
        obj.get(16);
        obj.get(74);
        obj.put(88, 251);
        obj.get(124);
        obj.put(58, 101);
        obj.put(42, 81);
        obj.get(2);
        obj.put(133, 101);
        obj.get(16);
        obj.put(1, 254);
        obj.put(25, 167);
        obj.put(53, 56);
        obj.put(73, 198);
        obj.get(48);
        obj.get(30);
        obj.get(95);
        obj.put(90, 102);
        obj.put(92, 56);
        obj.put(2, 130);
        obj.put(52, 11);
        obj.get(9);
        obj.get(23);
        obj.put(53, 275);
        obj.put(23, 258);
        obj.get(57);
        obj.put(136, 183);
        obj.put(75, 265);
        obj.get(85);
        obj.put(68, 274);
        obj.put(15, 255);
        obj.get(85);
        obj.put(33, 314);
        obj.put(101, 223);
        obj.put(39, 248);
        obj.put(18, 261);
        obj.put(37, 160);
        obj.get(112);
        obj.get(65);
        obj.put(31, 240);
        obj.put(40, 295);
        obj.put(99, 231);
        obj.get(123);
        obj.put(34, 43);
        obj.get(87);
        obj.get(80);
        obj.put(47, 279);
        obj.put(89, 299);
        obj.get(72);
        obj.put(26, 277);
        obj.put(92, 13);
        obj.put(46, 92);
        obj.put(67, 163);
        obj.put(85, 184);
        obj.get(38);
        obj.put(35, 65);
        obj.get(70);
        obj.get(81);
        obj.put(40, 65);
        obj.get(80);
        obj.put(80, 23);
        obj.put(76, 258);
        obj.get(69);
        obj.get(133);
        obj.put(123, 196);
        obj.put(119, 212);
        obj.put(13, 150);
        obj.put(22, 52);
        obj.put(20, 105);
        obj.put(61, 233);
        obj.get(97);
        obj.put(128, 307);
        obj.get(85);
        obj.get(80);
        obj.get(73);
        obj.get(30);
        obj.put(46, 44);
        obj.get(95);
        obj.put(121, 211);
        obj.put(48, 307);
        obj.get(2);
        obj.put(27, 166);
        obj.get(50);
        obj.put(75, 41);
        obj.put(101, 105);
        obj.get(2);
        obj.put(110, 121);
        obj.put(32, 88);
        obj.put(75, 84);
        obj.put(30, 165);
        obj.put(41, 142);
        obj.put(128, 102);
        obj.put(105, 90);
        obj.put(86, 68);
        obj.put(13, 292);
        obj.put(83, 63);
        obj.put(5, 239);
        obj.get(5);
        obj.put(68, 204);
        obj.get(127);
        obj.put(42, 137);
        obj.get(93);
        obj.put(90, 258);
        obj.put(40, 275);
        obj.put(7, 96);
        obj.get(108);
        obj.put(104, 91);
        obj.get(63);
        obj.get(31);
        obj.put(31, 89);
        obj.get(74);
        obj.get(81);
        obj.put(126, 148);
        obj.get(107);
        obj.put(13, 28);
        obj.put(21, 139);
        obj.get(114);
        obj.get(5);
        obj.get(89);
        obj.get(133);
        obj.get(20);
        obj.put(96, 135);
        obj.put(86, 100);
        obj.put(83, 75);
        obj.get(14);
        obj.put(26, 195);
        obj.get(37);
        obj.put(1, 287);
        obj.get(79);
        obj.get(15);
        obj.get(6);
        obj.put(68, 11);
        obj.get(52);
        obj.put(124, 80);
        obj.put(123, 277);
        obj.put(99, 281);
        obj.get(133);
        obj.get(90);
        obj.get(45);
        obj.get(127);
        obj.put(9, 68);
        obj.put(123, 6);
        obj.put(124, 251);
        obj.put(130, 191);
        obj.put(23, 174);
        obj.put(69, 295);
        obj.get(32);
        obj.get(37);
        obj.put(1, 64);
        obj.put(48, 116);
        obj.get(68);
        obj.put(117, 173);
        obj.put(16, 89);
        obj.get(84);
        obj.put(28, 234);
        obj.get(129);
        obj.get(89);
        obj.get(55);
        obj.get(83);
        obj.put(99, 264);
        obj.get(129);
        obj.get(84);
        obj.get(14);
        obj.put(26, 274);
        obj.get(109);
        obj.get(110);
        obj.put(96, 120);
        obj.put(128, 207);
        obj.get(12);
        obj.put(99, 233);
        obj.put(20, 305);
        obj.put(26, 24);
        obj.put(102, 32);
        obj.get(82);
        obj.put(16, 30);
        obj.put(5, 244);
        obj.get(130);
        obj.put(109, 36);
        obj.put(134, 162);
        obj.put(13, 165);
        obj.put(45, 235);
        obj.put(112, 80);
        obj.get(6);
        obj.put(34, 98);
        obj.put(64, 250);
        obj.put(18, 237);
        obj.put(72, 21);
        obj.put(42, 105);
        obj.put(57, 108);
        obj.put(28, 229);
        obj.get(83);
        obj.put(1, 34);
        obj.put(93, 151);
        obj.put(132, 94);
        obj.put(18, 24);
        obj.put(57, 68);
        obj.put(42, 137);
        obj.get(35);
        obj.get(80);
        obj.put(10, 288);
        obj.get(21);
        obj.get(115);
        obj.get(131);
        obj.get(30);
        obj.get(43);
        obj.put(97, 262);
        obj.put(55, 146);
        obj.put(81, 112);
        obj.put(2, 212);
        obj.put(5, 312);
        obj.put(82, 107);
        obj.put(14, 151);
        obj.get(77);
        obj.put(60, 42);
        obj.put(90, 309);
        obj.get(90);
        obj.put(131, 220);
        obj.get(86);
        obj.put(106, 85);
        obj.put(85, 254);
        obj.get(14);
        obj.put(66, 262);
        obj.put(88, 243);
        obj.get(3);
        obj.put(50, 301);
        obj.put(118, 91);
        obj.get(25);
        obj.get(105);
        obj.get(100);
        obj.get(89);
        obj.put(111, 152);
        obj.put(65, 24);
        obj.put(41, 264);
        obj.get(117);
        obj.get(117);
        obj.put(80, 45);
        obj.get(38);
        obj.put(11, 151);
        obj.put(126, 203);
        obj.put(128, 59);
        obj.put(6, 129);
        obj.get(91);
        obj.put(118, 2);
        obj.put(50, 164);
        obj.get(74);
        obj.get(80);
        obj.put(48, 308);
        obj.put(109, 82);
        obj.put(3, 48);
        obj.put(123, 10);
        obj.put(59, 249);
        obj.put(128, 64);
        obj.put(41, 287);
        obj.put(52, 278);
        obj.put(98, 151);
        obj.get(12);
        obj.get(25);
        obj.put(18, 254);
        obj.put(24, 40);
        obj.get(119);
        obj.put(66, 44);
        obj.put(61, 19);
        obj.put(80, 132);
        obj.put(62, 111);
        obj.get(80);
        obj.put(57, 188);
        obj.get(132);
        obj.get(42);
        obj.put(18, 314);
        obj.get(48);
        obj.put(86, 138);
        obj.get(8);
        obj.put(27, 88);
        obj.put(96, 178);
        obj.put(17, 104);
        obj.put(112, 86);
        obj.get(25);
        obj.put(129, 119);
        obj.put(93, 44);
        obj.get(115);
        obj.put(33, 36);
        obj.put(85, 190);
        obj.get(10);
        obj.put(52, 182);
        obj.put(76, 182);
        obj.get(109);
        obj.get(118);
        obj.put(82, 301);
        obj.put(26, 158);
        obj.get(71);
        obj.put(108, 309);
        obj.put(58, 132);
        obj.put(13, 299);
        obj.put(117, 183);
        obj.get(115);
        obj.get(89);
        obj.get(42);
        obj.put(11, 285);
        obj.put(30, 144);
        obj.get(69);
        obj.put(31, 53);
        obj.get(21);
        obj.put(96, 162);
        obj.put(4, 227);
        obj.put(77, 120);
        obj.put(128, 136);
        obj.get(92);
        obj.put(119, 208);
        obj.put(87, 61);
        obj.put(9, 40);
        obj.put(48, 273);
        obj.get(95);
        obj.get(35);
        obj.put(62, 267);
        obj.put(88, 161);
        obj.get(59);
        obj.get(85);
        obj.put(131, 53);
        obj.put(114, 98);
        obj.put(90, 257);
        obj.put(108, 46);
        obj.get(54);
        obj.put(128, 223);
        obj.put(114, 168);
        obj.put(89, 203);
        obj.get(100);
        obj.get(116);
        obj.get(14);
        obj.put(61, 104);
        obj.put(44, 161);
        obj.put(60, 132);
        obj.put(21, 310);
        obj.get(89);
        obj.put(109, 237);
        obj.get(105);
        obj.get(32);
        obj.put(78, 101);
        obj.put(14, 71);
        obj.put(100, 47);
        obj.put(102, 33);
        obj.put(44, 29);
        obj.get(85);
        obj.get(37);
        obj.put(68, 175);
        obj.put(116, 182);
        obj.put(42, 47);
        obj.get(9);
        obj.put(64, 37);
        obj.put(23, 32);
        obj.put(11, 124);
        obj.put(130, 189);
        obj.get(65);
        obj.put(33, 219);
        obj.put(79, 253);
        obj.get(80);
        obj.get(16);
        obj.put(38, 18);
        obj.put(35, 67);
        obj.get(107);
        obj.get(88);
        obj.put(37, 13);
        obj.put(71, 188);
        obj.get(35);
        obj.put(58, 268);
        obj.put(18, 260);
        obj.put(73, 23);
        obj.put(28, 102);
        obj.get(129);
        obj.get(88);
        obj.get(65);
        obj.get(80);
        obj.put(119, 146);
        obj.get(113);
        obj.get(62);
        obj.put(123, 138);
        obj.put(18, 1);
        obj.put(26, 208);
        obj.get(107);
        obj.get(107);
        obj.put(76, 132);
        obj.put(121, 191);
        obj.get(4);
        obj.get(8);
        obj.get(117);
        obj.put(11, 118);
        obj.get(43);
        obj.get(69);
        obj.get(136);
        obj.put(66, 298);
        obj.get(25);
        obj.get(71);
        obj.get(100);
        obj.put(26, 141);
        obj.put(53, 256);
        obj.put(111, 205);
        obj.put(126, 106);
        obj.get(43);
        obj.put(14, 39);
        obj.put(44, 41);
        obj.put(23, 230);
        obj.get(131);
        obj.get(53);
        obj.put(104, 268);
        obj.get(30);
        obj.put(108, 48);
        obj.put(72, 45);
        obj.get(58);
        obj.get(46);
        obj.put(128, 301);
        obj.get(71);
        obj.get(99);
        obj.get(113);
        obj.get(121);
        obj.put(130, 122);
        obj.put(102, 5);
        obj.put(111, 51);
        obj.put(85, 229);
        obj.put(86, 157);
        obj.put(82, 283);
        obj.put(88, 52);
        obj.put(136, 105);
        obj.get(40);
        obj.get(63);
        obj.put(114, 244);
        obj.put(29, 82);
        obj.put(83, 278);
        obj.get(131);
        obj.put(56, 33);
        obj.get(123);
        obj.get(11);
        obj.get(119);
        obj.put(119, 1);
        obj.put(48, 52);
        obj.get(47);
        obj.put(127, 136);
        obj.put(78, 38);
        obj.put(117, 64);
        obj.put(130, 134);
        obj.put(93, 69);
        obj.put(70, 98);
        obj.get(68);
        obj.put(4, 3);
        obj.put(92, 173);
        obj.put(114, 65);
        obj.put(7, 309);
        obj.get(31);
        obj.put(107, 271);
        obj.put(110, 69);
        obj.get(45);
        obj.put(35, 288);
        obj.get(20);
        obj.put(38, 79);
        obj.get(46);
        obj.put(6, 123);
        obj.get(19);
        obj.put(84, 95);
        obj.get(76);
        obj.put(71, 31);
        obj.put(72, 171);
        obj.put(35, 123);
        obj.get(32);
        obj.put(73, 85);
        obj.get(94);
        obj.get(128);
        obj.get(28);
        obj.get(38);
        obj.get(109);
        obj.put(85, 197);
        obj.put(10, 41);
        obj.put(71, 50);
        obj.get(128);
        obj.put(3, 55);
        obj.put(15, 9);
        obj.put(127, 215);
        obj.get(17);
        obj.get(37);
        obj.put(111, 272);
        obj.put(79, 169);
        obj.put(86, 206);
        obj.put(40, 264);
        obj.get(134);
        obj.put(16, 207);
        obj.put(27, 127);
        obj.put(29, 48);
        obj.put(32, 122);
        obj.put(15, 35);
        obj.put(117, 36);
        obj.get(127);
        obj.get(36);
        obj.put(72, 70);
        obj.put(49, 201);
        obj.put(89, 215);
        obj.put(134, 290);
        obj.put(77, 64);
        obj.put(26, 101);
        obj.get(99);
        obj.put(36, 96);
        obj.put(84, 129);
        obj.put(125, 264);
        obj.get(43);
        obj.get(38);
        obj.put(24, 76);
        obj.put(45, 2);
        obj.put(32, 24);
        obj.put(84, 235);
        obj.put(16, 240);
        obj.put(17, 289);
        obj.put(49, 94);
        obj.put(90, 54);
        obj.put(88, 199);
        obj.get(23);
        obj.put(87, 19);
        obj.put(11, 19);
        obj.get(24);
        obj.get(57);
        obj.get(4);
        obj.get(40);
        obj.put(133, 286);
        obj.put(127, 231);
        obj.get(51);
        obj.put(52, 196);
        obj.get(27);
        obj.get(10);
        obj.get(93);
        obj.put(115, 143);
        obj.put(62, 64);
        obj.put(59, 200);
        obj.put(75, 85);
        obj.put(7, 93);
        obj.put(117, 270);
        obj.put(116, 6);
        obj.get(32);
        obj.get(135);
        obj.put(2, 140);
        obj.put(23, 1);
        obj.put(11, 69);
        obj.put(89, 30);
        obj.put(27, 14);
        obj.get(100);
        obj.get(61);
        obj.put(99, 41);
        obj.put(88, 12);
        obj.get(41);
        obj.put(52, 203);
        obj.get(65);
        obj.put(62, 78);
        obj.put(104, 276);
        obj.put(105, 307);
        obj.get(7);
        obj.put(23, 123);
        obj.get(22);
        obj.put(35, 299);
        obj.get(69);
        obj.get(11);
        obj.put(14, 112);
        obj.get(115);
        obj.get(112);
        obj.get(108);
        obj.put(110, 165);
        obj.put(83, 165);
        obj.put(36, 260);
        obj.put(54, 73);
        obj.get(36);
        obj.put(93, 69);
        obj.get(134);
        obj.put(125, 96);
        obj.put(74, 127);
        obj.put(110, 305);
        obj.put(92, 309);
        obj.put(87, 45);
        obj.put(31, 266);
        obj.get(10);
        obj.put(114, 206);
        obj.put(49, 141);
        obj.get(82);
        obj.put(92, 3);
        obj.put(91, 160);
        obj.get(41);
        obj.put(60, 147);
        obj.put(36, 239);
        obj.put(23, 296);
        obj.put(134, 120);
        obj.get(6);
        obj.put(5, 283);
        obj.put(117, 68);
        obj.get(35);
        obj.get(120);
        obj.put(44, 191);
        obj.put(121, 14);
        obj.put(118, 113);
        obj.put(84, 106);
        obj.get(23);
        obj.put(15, 240);
        obj.get(37);
        obj.put(52, 256);
        obj.put(119, 116);
        obj.put(101, 7);
        obj.put(14, 157);
        obj.put(29, 225);
        obj.put(4, 247);
        obj.put(8, 112);
        obj.put(8, 189);
        obj.put(96, 220);
        obj.get(104);
        obj.put(72, 106);
        obj.put(23, 170);
        obj.put(67, 209);
        obj.put(70, 39);
        obj.get(18);
        obj.get(6);
        obj.get(34);
        obj.put(121, 157);
        obj.get(16);
        obj.get(19);
        obj.put(83, 283);
        obj.put(13, 22);
        obj.put(33, 143);
        obj.put(88, 133);
        obj.get(88);
        obj.put(5, 49);
        obj.get(38);
        obj.get(110);
        obj.get(67);
        obj.put(23, 227);
        obj.get(68);
        obj.get(3);
        obj.put(27, 265);
        obj.get(31);
        obj.put(13, 103);
        obj.get(116);
        obj.put(111, 282);
        obj.put(43, 71);
        obj.get(134);
        obj.put(70, 141);
        obj.get(14);
        obj.get(119);
        obj.get(43);
        obj.get(122);
        obj.put(38, 187);
        obj.put(8, 9);
        obj.get(63);
        obj.put(42, 140);
        obj.get(83);
        obj.get(92);
        obj.get(106);
        obj.get(28);
        obj.put(57, 139);
        obj.put(36, 257);
        obj.put(30, 204);
        obj.get(72);
        obj.put(105, 243);
        obj.get(16);
        obj.put(74, 25);
        obj.get(22);
        obj.put(118, 144);
        obj.get(133);
        obj.get(71);
        obj.put(99, 21);
        obj.get(26);
        obj.get(35);
        obj.put(89, 209);
        obj.put(106, 158);
        obj.put(76, 63);
        obj.put(112, 216);
        obj.get(128);
        obj.get(54);
        obj.put(16, 165);
        obj.put(76, 206);
        obj.put(69, 253);
        obj.get(23);
        obj.put(54, 111);
        obj.get(80);
        obj.put(111, 72);
        obj.put(95, 217);
        obj.get(118);
        obj.put(4, 146);
        obj.get(47);
        obj.put(108, 290);
        obj.get(43);
        obj.put(70, 8);
        obj.get(117);
        obj.get(121);
        obj.put(42, 220);
        obj.get(48);
        obj.get(32);
        obj.put(68, 213);
        obj.put(30, 157);
        obj.put(62, 68);
        obj.get(58);
        obj.put(125, 283);
        obj.put(132, 45);
        obj.get(85);
        obj.get(92);
        obj.put(23, 257);
        obj.get(74);
        obj.put(18, 256);
        obj.get(90);
        obj.put(10, 158);
        obj.put(57, 34);
        obj.get(27);
        obj.get(107);
    }
}
