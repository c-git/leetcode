struct MyHashSet {
    keys: Vec<bool>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {
    const MAX_KEY: usize = 1_000_001;

    fn new() -> Self {
        Self {
            keys: vec![false; Self::MAX_KEY],
        }
    }

    fn add(&mut self, key: i32) {
        Self::check_input_validity(key);
        self.keys[key as usize] = true;
    }

    fn remove(&mut self, key: i32) {
        Self::check_input_validity(key);
        self.keys[key as usize] = false;
    }

    fn contains(&self, key: i32) -> bool {
        Self::check_input_validity(key);
        self.keys[key as usize]
    }

    #[inline]
    fn check_input_validity(key: i32) {
        debug_assert!(key < Self::MAX_KEY as i32);
        debug_assert!(key >= 0);
    }
}

/**
 * Your MyHashSet object will be instantiated and called as such:
 * let obj = MyHashSet::new();
 * obj.add(key);
 * obj.remove(key);
 * let ret_3: bool = obj.contains(key);
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut my_hash_set = MyHashSet::new();
        my_hash_set.add(1); // set = [1]
        my_hash_set.add(2); // set = [1, 2]
        assert!(my_hash_set.contains(1)); // return True
        assert!(!my_hash_set.contains(3)); // return False, (not found)
        my_hash_set.add(2); // set = [1, 2]
        assert!(my_hash_set.contains(2)); // return True
        my_hash_set.remove(2); // set = [1]
        assert!(!my_hash_set.contains(2)); // return False, (already removed)
    }

    #[test]
    fn case2() {
        let mut my_hash_set = MyHashSet::new();
        assert!(!my_hash_set.contains(0));
        assert!(!my_hash_set.contains(1_000_000));
        my_hash_set.add(0);
        my_hash_set.add(1_000_000);
        assert!(my_hash_set.contains(0));
        assert!(my_hash_set.contains(1_000_000));
        my_hash_set.remove(0);
        my_hash_set.remove(1_000_000);
        assert!(!my_hash_set.contains(0));
        assert!(!my_hash_set.contains(1_000_000));
    }

    #[test]
    #[should_panic]
    fn case3() {
        let mut my_hash_set = MyHashSet::new();
        my_hash_set.add(-1);
    }

    #[test]
    #[should_panic]
    fn case4() {
        let mut my_hash_set = MyHashSet::new();
        my_hash_set.add(1_000_001);
    }
}
