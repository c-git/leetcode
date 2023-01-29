use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};

type TimeStamp = usize;

fn get_first(set: &BTreeSet<Element>) -> Option<Element> {
    let mut result = None;
    if let Some(x) = set.iter().next() {
        result = Some(x.clone());
    }
    result
}

#[derive(Debug, Clone)]
struct Element {
    key: i32,
    value: i32,
    id: TimeStamp, //Assumption: Globally No two elements can have the same id
    count: u32,
}

impl Element {
    pub fn increment_count(&mut self, new_timestamp: usize) {
        assert!(new_timestamp > self.id);
        self.id = new_timestamp;
        self.count += 1;
    }
}

impl Eq for Element {}

impl PartialEq<Self> for Element {
    fn eq(&self, other: &Self) -> bool {
        // Uses assumption that IDs are unique
        self.id == other.id
    }
}

impl PartialOrd<Self> for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }

        match self.count.cmp(&other.count) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                assert_ne!(self.id.cmp(&other.id), Ordering::Equal);
                self.id.cmp(&other.id)
            }
            Ordering::Greater => Ordering::Greater,
        }
    }
}

#[derive(Debug)]
struct LFUCache {
    capacity: usize,
    keys: HashMap<i32, Element>,
    values: BTreeSet<Element>,
    timestamp: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LFUCache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as usize,
            keys: Default::default(),
            values: Default::default(),
            timestamp: 0,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        self.timestamp += 1;
        match self.keys.get_mut(&key) {
            None => -1,
            Some(element) => {
                assert!(
                    self.values.remove(element),
                    "Invariant broken. Value should exist in both"
                );
                element.increment_count(self.timestamp);
                self.values.insert(element.clone());
                let result = element.value;
                assert_eq!(self.keys.len(), self.values.len());
                result
            }
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            // Do nothing no space to store}
            return;
        }

        self.timestamp += 1;
        let removed = if self.capacity <= self.values.len() {
            // Need to remove a value
            get_first(&self.values)
        } else {
            None
        };

        // Remove value, cannot use pop_first on rust 1.58 which LeetCode uses
        if let Some(x) = &removed {
            self.values.remove(x);
        }

        let element = match self.keys.get_mut(&key) {
            None => {
                // Create new element because key is new
                self.keys.insert(
                    key,
                    Element {
                        key,
                        value,
                        id: self.timestamp,
                        count: 1,
                    },
                );
                self.keys
                    .get(&key)
                    .expect("Should have been here just inserted")
                    .clone()
            }
            Some(element) => {
                match &removed {
                    Some(x) if x == element => {
                        // After match statement this key will need to be removed from self.keys
                    }

                    None | Some(_) => {
                        // Nothing removed or not this key so remove element for this key
                        assert!(
                            self.values.remove(element),
                            "Invariant broken. Value should exist in both"
                        );
                    }
                }
                element.increment_count(self.timestamp);
                element.value = value;
                element.clone()
            }
        };
        self.values.insert(element);
        if let Some(x) = removed {
            if x.key != key {
                // The overflow value is not the one that was updated so remove from keys
                self.keys.remove(&x.key);
            }
        }
        assert_eq!(self.keys.len(), self.values.len());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        // cnt(x) = the use counter for key x
        // cache=[] will show the last used order for tiebreakers (leftmost element is  most recent)
        let mut lfu = LFUCache::new(2);
        lfu.put(1, 1); // cache=[1,_], cnt(1)=1
        lfu.put(2, 2); // cache=[2,1], cnt(2)=1, cnt(1)=1
        assert_eq!(lfu.get(1), 1); // return 1
                                   // cache=[1,2], cnt(2)=1, cnt(1)=2
        lfu.put(3, 3); // 2 is the LFU key because cnt(2)=1 is the smallest, invalidate 2.
                       // cache=[3,1], cnt(3)=1, cnt(1)=2
        assert_eq!(lfu.get(2), -1); // return -1 (not found)
        assert_eq!(lfu.get(3), 3); // return 3
                                   // cache=[3,1], cnt(3)=2, cnt(1)=2
        lfu.put(4, 4); // Both 1 and 3 have the same cnt, but 1 is LRU, invalidate 1.
                       // cache=[4,3], cnt(4)=1, cnt(3)=2
        assert_eq!(lfu.get(1), -1); // return -1 (not found)
        assert_eq!(lfu.get(3), 3); // return 3
                                   // cache=[3,4], cnt(4)=1, cnt(3)=3
        assert_eq!(lfu.get(4), 4); // return 4
                                   // cache=[4,3], cnt(4)=2, cnt(3)=3
    }

    #[test]
    fn empty_capacity() {
        let mut lfu = LFUCache::new(0);
        lfu.put(0, 0);
        assert_eq!(lfu.get(0), -1);
    }

    #[test]
    fn empty_failed_case2() {
        let mut lfu = LFUCache::new(2);
        lfu.put(2, 1);
        lfu.put(2, 2);
        assert_eq!(lfu.get(2), 2);
        lfu.put(1, 1);
        lfu.put(4, 1);
        assert_eq!(lfu.get(2), 2);
    }
}
/*
 * Your LFUCache object will be instantiated and called as such:
 * let obj = LFUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
