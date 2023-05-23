struct KthLargest {
    k: usize,
    k_top: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let k = k as usize;
        debug_assert!(nums.len() >= k - 1);
        Self { k, k_top: nums }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.k_top.push(val);
        self.k_top.sort_unstable_by(|a, b| b.cmp(a));
        if self.k_top.len() > self.k {
            self.k_top.drain(self.k..);
        }
        self.k_top[self.k - 1]
    }
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut kth_largest = KthLargest::new(3, [4, 5, 8, 2].into());
        assert_eq!(kth_largest.add(3), 4); // return 4
        assert_eq!(kth_largest.add(5), 5); // return 5
        assert_eq!(kth_largest.add(10), 5); // return 5
        assert_eq!(kth_largest.add(9), 8); // return 8
        assert_eq!(kth_largest.add(4), 8); // return 8
    }
}
