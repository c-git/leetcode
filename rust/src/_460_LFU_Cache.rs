struct LFUCache {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LFUCache {
    fn new(capacity: i32) -> Self {
        todo!()
    }

    fn get(&self, key: i32) -> i32 {
        todo!()
    }

    fn put(&self, key: i32, value: i32) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        // cnt(x) = the use counter for key x
        // cache=[] will show the last used order for tiebreakers (leftmost element is  most recent)
        let lfu = LFUCache::new(2);
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
}
/*
 * Your LFUCache object will be instantiated and called as such:
 * let obj = LFUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
