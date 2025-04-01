//! Solution for https://leetcode.com/problems/design-hashmap
//! 706. Design HashMap

struct MyHashMap {
    data: Vec<Vec<(i32, i32)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {
    fn new() -> Self {
        let size = 100_000;
        Self {
            data: vec![Default::default(); size],
        }
    }

    fn backing_index(&self, key: i32) -> usize {
        key.unsigned_abs() as usize % self.data.len()
    }

    fn put(&mut self, key: i32, value: i32) {
        let index = self.backing_index(key);
        if let Some(prev) = self.data[index]
            .iter_mut()
            .find(|(existing_key, _)| *existing_key == key)
        {
            prev.1 = value;
        } else {
            self.data[index].push((key, value));
        }
    }

    fn get(&self, key: i32) -> i32 {
        let index = self.backing_index(key);
        if let Some(prev) = self.data[index]
            .iter()
            .find(|(existing_key, _)| *existing_key == key)
        {
            prev.1
        } else {
            -1
        }
    }

    fn remove(&mut self, key: i32) {
        let index = self.backing_index(key);
        self.data[index].retain(|(existing_key, _)| existing_key != &key);
    }
}

/*
 * Your MyHashMap object will be instantiated and called as such:
 * let obj = MyHashMap::new();
 * obj.put(key, value);
 * let ret_2: i32 = obj.get(key);
 * obj.remove(key);
 */

// << ---------------- Code below here is only for local use ---------------- >>

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn manual() {
        let mut my_hash_map = MyHashMap::new();
        my_hash_map.put(1, 1); // The map is now [[1,1]]
        my_hash_map.put(2, 2); // The map is now [[1,1], [2,2]]
        assert_eq!(my_hash_map.get(1), 1); // return 1, The map is now [[1,1], [2,2]]
        assert_eq!(my_hash_map.get(3), -1); // return -1 (i.e., not found), The map is now [[1,1], [2,2]]
        my_hash_map.put(2, 1); // The map is now [[1,1], [2,1]] (i.e., update the existing value)
        assert_eq!(my_hash_map.get(2), 1); // return 1, The map is now [[1,1], [2,1]]
        my_hash_map.remove(2); // remove the mapping for 2, The map is now [[1,1]]
        assert_eq!(my_hash_map.get(2), -1); // return -1 (i.e., not found), The map is now [[1,1]]
        for x in 5..20 {
            my_hash_map.put(x, x + 5);
            assert_eq!(my_hash_map.get(x), x + 5);
        }
    }
}
